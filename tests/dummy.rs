use axum_sqlx_template::http;
use sqlx::PgPool;

use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use std::borrow::BorrowMut;

mod common;

use common::{response_json, RequestBuilderExt};

use serde_json::json;

#[sqlx::test]
async fn test_dummy_post_method(db: PgPool) {
    let mut app = http::app(db);

    // Happy path!
    let resp1 = app
        .borrow_mut()
        // We handle JSON objects directly to sanity check the serialization and deserialization
        .oneshot(Request::post("/dummy").json(json! {{
            "field": "maishahalisi",
        }}))
        .await
        .unwrap();

    assert_eq!(resp1.status(), StatusCode::OK);
}
