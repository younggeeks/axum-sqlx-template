use anyhow::Context;
use axum::{Extension, Router};
pub use error::Error;
use sqlx::PgPool;

mod error;

pub mod hello;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

pub fn app(db: PgPool) -> Router {
    Router::new().merge(hello::router()).layer(Extension(db))
}

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    let server = axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .context("Failed to serve API");

    println!("Server running at http://0.0.0.0:8080");

    server
}
