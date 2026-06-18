use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::env;

use my_stackoverflow_api::{
    api_axum::{
        app_state::AppState,
        handlers::{
            create_answer, create_question, delete_answer, delete_question, read_answers,
            read_questions,
        },
    },
    persistence::{answers_dao::AnswersDaoImpl, questions_dao::QuestionsDaoImpl},
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create Postgres connection pool!");

    let app_state = AppState::new(
        QuestionsDaoImpl::new(pool.clone()),
        AnswersDaoImpl::new(pool),
    );

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
