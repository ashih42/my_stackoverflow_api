use std::sync::Arc;

use crate::persistence::{answers_dao::AnswersDao, questions_dao::QuestionsDao};

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao>,
    pub answers_dao: Arc<dyn AnswersDao>,
}

impl AppState {
    pub fn new(
        questions_dao: impl QuestionsDao + 'static,
        answers_dao: impl AnswersDao + 'static,
    ) -> Self {
        Self {
            questions_dao: Arc::new(questions_dao),
            answers_dao: Arc::new(answers_dao),
        }
    }
}
