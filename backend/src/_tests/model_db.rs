use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    // CHECK
    let result = sqlx::query("SELECT * from ingredients")
        .fetch_all(&db)
        .await?;
    assert_eq!(1, result.len(), "number of seed ingredients");

    let result = sqlx::query("SELECT * from recipes").fetch_all(&db).await?;
    assert_eq!(1, result.len(), "number of seed recipes");

    Ok(())
}
