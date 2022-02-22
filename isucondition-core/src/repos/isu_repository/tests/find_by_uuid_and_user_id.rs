use crate::database::get_db_connection_for_test;
use crate::models::isu::IsuUUID;
use crate::models::user::UserID;
use crate::repos;
use crate::repos::isu_repository::{IsuRepository, IsuRepositoryImpl};
use crate::test::Cleaner;
use repos::Result;

#[tokio::test]
async fn with_empty() -> Result<()> {
    let pool = get_db_connection_for_test().await;

    let mut cleaner = Cleaner::new(pool.clone());
    cleaner.prepare_table("isu").await?;

    let repo = IsuRepositoryImpl { pool: pool };
    let result = repo
        .find_by_uuid_and_user_id(
            &IsuUUID::new("test".to_string()),
            &UserID::new("1".to_string()),
        )
        .await?;

    assert!(result.is_none());

    cleaner.clean().await?;

    Ok(())
}

#[tokio::test]
async fn with_result() -> Result<()> {
    let pool = get_db_connection_for_test().await;

    let mut cleaner = Cleaner::new(pool.clone());
    cleaner.prepare_table("isu").await?;

    sqlx::query!(
        "INSERT INTO isu (jia_user_id, jia_isu_uuid, name) VALUES  (?,?,?)",
        "1".to_string(),
        "test".to_string(),
        "test1".to_string(),
    )
    .execute(&pool)
    .await?;

    let repo = IsuRepositoryImpl { pool: pool };
    let result = repo
        .find_by_uuid_and_user_id(
            &IsuUUID::new("test".to_string()),
            &UserID::new("1".to_string()),
        )
        .await?;

    assert!(result.is_some());

    cleaner.clean().await?;

    Ok(())
}
