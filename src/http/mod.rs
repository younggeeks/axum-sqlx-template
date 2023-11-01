use anyhow::Context;
use axum::{Extension, Router};
use sqlx::PgPool;

pub fn app(db: PgPool) -> Router {
    Router::new().layer(Extension(db))
}

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .context("Failed to serve API")
}
