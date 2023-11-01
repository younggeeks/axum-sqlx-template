use crate::http::{Error, Result};
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DummyData {
    #[validate(length(
        min = 8,
        max = 32,
        message = "This field can't have less than 8 and more than 32 characters"
    ))]
    field: String,
}

pub fn router() -> Router {
    Router::new().route("/dummy", post(dummy_post))
}

pub async fn dummy_post(db: Extension<PgPool>, Json(req): Json<DummyData>) -> Result<StatusCode> {
    req.validate()?;

    let DummyData { field } = req;

    sqlx::query!(
        r#"
            insert into "dummy"(field)
            values ($1)
        "#,
        field,
    )
    .execute(&*db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_username_key") => {
            Error::Conflict("username taken".into())
        }
        _ => e.into(),
    })?;

    Ok(StatusCode::OK)
}
