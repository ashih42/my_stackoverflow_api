use async_trait::async_trait;
use sqlx::{PgPool, types::Uuid};

use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = Uuid::parse_str(&answer.question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!(
                "Could not parse answer UUID: {}",
                answer.question_uuid
            ))
        })?;

        let answer = sqlx::query_as!(
            AnswerDetail,
            r#"
                INSERT INTO answers ( question_uuid, content )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| {
            if let sqlx::Error::Database(database_err) = &err
                && let Some(code) = database_err.code()
                && code == postgres_error_codes::FOREIGN_KEY_VIOLATION
            {
                return DBError::InvalidUUID(format!(
                    "Invalid question uuid: {}",
                    answer.question_uuid
                ));
            }
            DBError::Other(Box::new(err))
        })?;

        Ok(answer)
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(&answer_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse answer UUID: {}", answer_uuid))
        })?;

        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::parse_str(&question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", question_uuid))
        })?;

        let answers = sqlx::query_as!(
            AnswerDetail,
            "SELECT * FROM answers WHERE question_uuid = $1",
            uuid
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(answers)
    }
}
