extern crate fakeit;

mod common;

use back_office::articles::entities::Slug;
use claim::{assert_none, assert_some};
use common::spawn_test_app;
use fakeit::image::url;
use fakeit::unique::uuid_v4;
use fakeit::words::{paragraph, sentence};
use serde_json::json;
use sqlx::query;

#[tokio::test]
async fn creation_fails_with_empty_slug() {
    let test_app = spawn_test_app().await;
    let body = json!({
        "slug": "",
        "title": sentence(5),
        "summary": paragraph(1, 1, 100, "".into()),
        "content": paragraph(1, 1, 1000, "".into()),
        "author_id": uuid_v4(),
    });

    let response = test_app.post_json("/api/articles", body).await;
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn creation_fails_with_empty_title() {
    let test_app = spawn_test_app().await;
    let body = json!({
        "slug": Slug::from(sentence(10)).as_ref(),
        "title": "",
        "summary": paragraph(1, 1, 100, "".into()),
        "content": paragraph(1, 1, 1000, "".into()),
        "author_id": uuid_v4(),
    });

    let response = test_app.post_json("/api/articles", body).await;
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn creation_fails_with_empty_summary() {
    let test_app = spawn_test_app().await;
    let body = json!({
        "slug": Slug::from(sentence(10)).as_ref(),
        "title": sentence(5),
        "summary": "",
        "content": paragraph(1, 1, 1000, "".into()),
        "author_id": uuid_v4(),
    });

    let response = test_app.post_json("/api/articles", body).await;
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn creation_fails_with_empty_content() {
    let test_app = spawn_test_app().await;
    let body = json!({
        "slug": Slug::from(sentence(10)).as_ref(),
        "title": sentence(5),
        "summary": paragraph(1, 1, 100, "".into()),
        "content": "",
        "author_id": uuid_v4(),
    });

    let response = test_app.post_json("/api/articles", body).await;
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn creation_succeeds_with_valid_data() {
    let test_app = spawn_test_app().await;
    let response = test_app.create_article().await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn creation_sets_status_to_draft_by_default() {
    let test_app = spawn_test_app().await;
    let _ = test_app.create_article().await;
    let article = query!(r#"SELECT status as "status: String" FROM articles"#)
        .fetch_one(&test_app.connection_pool)
        .await
        .unwrap();

    assert_eq!(article.status, "draft");
}

#[tokio::test]
async fn creation_sets_likes_to_zero_by_default() {
    let test_app = spawn_test_app().await;
    let _ = test_app.create_article().await;
    let article = query!(r#"SELECT likes FROM articles"#,)
        .fetch_one(&test_app.connection_pool)
        .await
        .unwrap();

    assert_eq!(article.likes, 0);
}

#[tokio::test]
async fn creation_persists_the_article_in_the_database() {
    let test_app = spawn_test_app().await;
    let slug = Slug::from(sentence(10));
    let title = sentence(5);
    let author_id = uuid_v4();
    let summary = paragraph(5, 5, 5, "".into());
    let content = paragraph(10, 20, 20, "".into());
    let body = json!({
        "slug": slug.as_ref(),
        "title": &title,
        "summary": &summary,
        "content": &content,
        "author_id": &author_id,
        "thumbnail_uri": "/thumbnail.jpg",
    });

    test_app.post_json("/api/articles", body.clone()).await;

    let saved = query!(
        "SELECT id, slug, title, summary, content, author_id, created_at, updated_at FROM articles"
    )
    .fetch_one(&test_app.connection_pool)
    .await
    .unwrap();

    assert_eq!(saved.slug, slug.as_ref());
    assert_eq!(saved.title, title);
    assert_eq!(saved.summary, summary);
    assert_eq!(saved.content, content);
    assert_eq!(saved.author_id.to_string(), author_id);
    assert_ne!(saved.created_at.to_string(), "");
    assert_none!(saved.updated_at);
}
