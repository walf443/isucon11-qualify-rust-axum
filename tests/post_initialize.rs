mod helper;

#[tokio::test]
async fn test_post_initialize() {
    std::env::set_var("MYSQL_DBNAME", std::env::var("MYSQL_DBNAME_TEST").unwrap_or_else(|_| "isucondition_test".to_owned() ));
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();
    let res = client.post(app.url.join("/initialize").unwrap()).send().await
        .expect("Failed to request");

    assert!(res.status().is_success());

    let result = sqlx::query!("SELECT COUNT(*) as count from isu_association_config").fetch_one(&app.database).await;
    assert_eq!(1, result.unwrap().count, "isu_association_config record created");

}