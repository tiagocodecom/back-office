extern crate fakeit;

mod common;

use common::spawn_test_app;

#[tokio::test]
async fn returns_the_login_page() {
    let test_app = spawn_test_app().await;
    let response = test_app.get("/admin/auth/login").await;

    assert_eq!(200, response.status().as_u16());

    let page_content = response.text().await.unwrap();
    assert!(!page_content.is_empty());
}
