use rocket::{Responder, State, delete, get, post, serde::json::Json};

use crate::{
    models::*,
    persistence::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};

mod handlers_inner;
use handlers_inner::*;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),

    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao>>,
) -> Result<Json<QuestionDetail>, APIError> {
    let question =
        handlers_inner::create_question(question.into_inner(), questions_dao.as_ref()).await?;

    Ok(Json(question))
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao>>,
) -> Result<Json<Vec<QuestionDetail>>, APIError> {
    let questions = handlers_inner::read_questions(questions_dao.as_ref()).await?;

    Ok(Json(questions))
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao>>,
) -> Result<(), APIError> {
    handlers_inner::delete_question(question_uuid.into_inner(), questions_dao.as_ref()).await?;

    Ok(())
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao>>,
) -> Result<Json<AnswerDetail>, APIError> {
    let answer = handlers_inner::create_answer(answer.into_inner(), answers_dao.as_ref()).await?;

    Ok(Json(answer))
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao>>,
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    let answers =
        handlers_inner::read_answers(question_uuid.into_inner(), answers_dao.as_ref()).await?;

    Ok(Json(answers))
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao>>,
) -> Result<(), APIError> {
    handlers_inner::delete_answer(answer_uuid.into_inner(), answers_dao.as_ref()).await?;

    Ok(())
}
