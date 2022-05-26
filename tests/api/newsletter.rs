use std::time::Duration;

use crate::helpers::{assert_is_redirect_to, spawn_app};

use uuid::Uuid;
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    let app = spawn_app().await;
    app.create_unconfirmed_subscriber().await;
    app.test_user.login(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    let app = spawn_app().await;
    app.create_confirmed_subscriber().await;
    app.test_user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });
    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
    // drop(Mock) verifies that we have sent the newsletter email
}

#[tokio::test]
async fn you_must_be_logged_in_to_publish_a_newsletter() {
    let app = spawn_app().await;

    let response = app
        .post_newsletters(&serde_json::json!({
            "title": "Newsletter title!",
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as HTML</p>",
            "idempotency_key": Uuid::new_v4().to_string(),
        }))
        .await;

    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_newsletter_form() {
    let app = spawn_app().await;
    let response = app.get_newsletters().await;
    assert_is_redirect_to(&response, "/login")
}

#[tokio::test]
async fn newsletter_creating_is_idempotent() {
    let app = spawn_app().await;
    app.create_confirmed_subscriber().await;
    // Login
    app.test_user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });
    // Send newsletter
    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    // Verify the newsletter has been published
    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));

    // Send newsletter again
    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    // Check if it's been published again
    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
}

#[tokio::test]
async fn concurrent_form_submission_is_handled_gracefully() {
    let app = spawn_app().await;
    app.create_confirmed_subscriber().await;
    app.test_user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2)))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });
    let response1 = app.post_newsletters(&newsletter_request_body);
    let response2 = app.post_newsletters(&newsletter_request_body);
    let (response1, response2) = tokio::join!(response1, response2);

    assert_eq!(response1.status(), response2.status());
    assert_eq!(
        response1.text().await.unwrap(),
        response2.text().await.unwrap()
    );
}
