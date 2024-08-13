mod common;

use common::spawn_test_app;
use serde_json::{json, Value};
use sqlx::query;
use uuid::Uuid;

#[tokio::test]
async fn should_return_200_for_valid_json_data() {
    let test_app = spawn_test_app().await;
    let body = json!({
        "author_id": Uuid::new_v4().to_string(),
        "title": "How to train your dragon",
        "content": "Very carefully.",
    });

    let response = test_app.post_json("/api/articles", body).await;
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn should_return_400_for_invalid_json_data() {
    let test_app = spawn_test_app().await;
    let cases = vec![
        json!({
            "author_id": "invalid_uuid",
            "title": "How to train your dragon",
            "content": "Very carefully.",
        }),
        json!({
            "author_id": Uuid::new_v4(),
            "title": "",
            "content": "Very carefully.",
        }),
    ];

    for body in cases {
        let response = test_app.post_json("/api/articles", body).await;
        assert_eq!(400, response.status().as_u16());
    }
}

#[tokio::test]
async fn should_persist_the_new_article_in_the_database() {
    let test_app = spawn_test_app().await;
    let author_id = Uuid::new_v4();

    let body = json!({
        "author_id": author_id.to_string(),
        "title": "How to train your dragon",
        "content": "Very carefully.",
    });

    test_app.post_json("/api/articles", body).await;

    let saved = query!("SELECT id, author_id, title, content FROM articles")
        .fetch_one(&test_app.connection_pool)
        .await
        .expect("Failed to fetch article from the database.");

    assert!(!saved.id.is_nil());
    assert_eq!(saved.author_id, author_id);
    assert_eq!(saved.title, "How to train your dragon");
    assert_eq!(saved.content, "Very carefully.");
}
