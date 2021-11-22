//! tests/health_check.rs

// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[actix_rt::test]
async fn health_check_works() {
    //Arrange
    spawn_app().await.expect("Failed to spawn app");
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to call health check endpoint");
    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
async fn spawn_app() -> std::io::Result<()> {
    devapi::run().await
}
