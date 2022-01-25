mod helper;

#[tokio::test]
async fn test_get_index() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();
    let res = client.get(&format!("{}/", &app.address)).send().await
        .expect("Failed to request");

    assert!(res.status().is_success());
}