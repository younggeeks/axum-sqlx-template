use anyhow::Context;
use axum_sqlx_template::http::serve;
use sqlx::postgres::PgPoolOptions;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = dotenvy::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("Failed to connect to DATABASE_UR")?;

    sqlx::migrate!().run(&db).await?;

    serve(db).await
}
