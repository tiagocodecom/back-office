extern crate fakeit;

mod common;

use common::spawn_test_app;
use fakeit::unique::uuid_v4;
use fakeit::words::{paragraph, sentence};
use serde_json::{json, Value};
use sqlx::query;

#[tokio::test]
async fn creation_returns_200_status_code_when_valid_data() {
    let test_app = spawn_test_app().await;
    let body = json!({
        "title": sentence(5),
        "author_id": uuid_v4(),
        "content": paragraph(5, 5, 10, "".into()),
    });

    let response = test_app.post_json("/api/articles", body).await;
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn creation_returns_400_status_code_when_invalid_data() {
    let test_app = spawn_test_app().await;
    let cases = vec![
        json!({
            "author_id": "invalid_author_uuid",
            "title": "How to train your dragon",
            "content": "Very carefully.",
        }),
        json!({
             "title": "",
             "author_id": uuid_v4(),
             "content": "",
        }),
    ];

    for body in cases {
        let response = test_app.post_json("/api/articles", body).await;
        assert_eq!(400, response.status().as_u16());
    }
}

#[tokio::test]
async fn creation_persists_the_article() {
    let test_app = spawn_test_app().await;
    let title = sentence(5);
    let author_id = uuid_v4();
    let content = paragraph(5, 5, 10, "".into());

    let body = json!({
        "title": &title,
        "author_id": &author_id,
        "content": &content,
    });

    test_app.post_json("/api/articles", body.clone()).await;

    let saved = query!("SELECT author_id, title, content FROM articles")
        .fetch_one(&test_app.connection_pool)
        .await
        .expect("Failed to fetch article from the database.");

    assert_eq!(saved.title, title);
    assert_eq!(saved.content, content);
    assert_eq!(saved.author_id.to_string(), author_id);
}
