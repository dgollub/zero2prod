fn spawn_app() {
    let server = zero2prod::create_server().expect("could not create server");
    // NOTE(dkg): launch server as background task
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn healthcheck_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/healthcheck")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length()); // "ok"
}
