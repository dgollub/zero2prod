fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind TCP socket to a free port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::create_server(listener).expect("could not create server");

    // NOTE(dkg): launch server as background task
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn healthcheck_works() {
    let base_url = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{base_url}/healthcheck"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length()); // "ok"
}
