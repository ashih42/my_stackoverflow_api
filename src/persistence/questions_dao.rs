use async_trait::async_trait;
use sqlx::{PgPool, types::Uuid};

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao: Send + Sync {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        QuestionsDaoImpl { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let question = sqlx::query_as!(
            QuestionDetail,
            r#"
                INSERT INTO questions ( title, description )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            question.title,
            question.description
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(question)
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(&question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", question_uuid))
        })?;

        sqlx::query!("DELETE FROM questions WHERE question_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let questions = sqlx::query_as!(QuestionDetail, "SELECT * FROM questions")
            .fetch_all(&self.db)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(questions)
    }
}
