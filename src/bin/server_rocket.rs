use rocket::{launch, routes};
use sqlx::postgres::PgPoolOptions;
use std::env;

use my_stackoverflow_api::{
    api_rocket::{
        cors::CORS,
        handlers::{
            create_answer, create_question, delete_answer, delete_question, read_answers,
            read_questions,
        },
    },
    persistence::{
        answers_dao::{AnswersDao, AnswersDaoImpl},
        questions_dao::{QuestionsDao, QuestionsDaoImpl},
    },
};

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create Postgres connection pool.");

    let questions_dao = QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
        .manage(Box::new(questions_dao) as Box<dyn QuestionsDao + Send + Sync>)
        .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Send + Sync>)
}
