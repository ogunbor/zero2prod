//! tests/health_check.rs

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    
    let client = reqwest::Client::new();
    
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
fn spawn_app() {
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            zero2prod::run("127.0.0.1:8000")
                .await
                .expect("Failed to start server");
        });
    });
    
    // Give server time to start
    std::thread::sleep(std::time::Duration::from_millis(200));
}