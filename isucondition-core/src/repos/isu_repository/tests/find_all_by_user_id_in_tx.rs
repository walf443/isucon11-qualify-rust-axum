use crate::database::get_db_connection_for_test;
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

    let mut tx = pool.begin().await?;

    let repo = IsuRepositoryImpl { pool: pool };
    let result = repo
        .find_all_by_user_id_in_tx(&mut tx, &UserID::new("1".to_string()))
        .await?;
    tx.commit().await?;

    assert_eq!(result.len(), 0);

    cleaner.clean().await?;

    Ok(())
}

#[tokio::test]
async fn with_result() -> Result<()> {
    let pool = get_db_connection_for_test().await;

    let mut cleaner = Cleaner::new(pool.clone());
    cleaner.prepare_table("isu").await?;

    sqlx::query!(
        "INSERT INTO isu (jia_user_id, jia_isu_uuid, name) VALUES  (?,?,?), (?,?,?), (?,?,?)",
        "1".to_string(),
        "xxxx".to_string(),
        "test1".to_string(),
        "1".to_string(),
        "yyyy".to_string(),
        "test2".to_string(),
        "1".to_string(),
        "zzzz".to_string(),
        "test3".to_string(),
    )
    .execute(&pool)
    .await?;

    let mut tx = pool.begin().await?;

    let repo = IsuRepositoryImpl { pool: pool };
    let result = repo
        .find_all_by_user_id_in_tx(&mut tx, &UserID::new("1".to_string()))
        .await?;

    tx.commit().await?;
    assert_eq!(result.len(), 3);

    cleaner.clean().await?;

    Ok(())
}
