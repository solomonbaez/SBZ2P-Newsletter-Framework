use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn failure_sends_error_flash_message() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"Authentication failed"#));

    let html_page = app.get_login_html().await;
    assert!(!html_page.contains("Authentication Failed"));
}

#[tokio::test]
async fn success_redirect_to_admin_dashboard() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}
