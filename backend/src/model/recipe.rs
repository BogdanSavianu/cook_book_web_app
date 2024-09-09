use super::db::{self, Db};
use crate::{model, security::UserCtx};
use chrono::{DateTime, Utc};
use sqlx::mysql;

// region: Recipe Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub cid: i64,
    pub ctime: DateTime<Utc>,
    pub mtime: DateTime<Utc>,
}

#[derive(Default, Clone)]
pub struct RecipePatch {
    pub title: Option<String>,
    pub cid: Option<i64>,
}
// endregion: Recipe Types

// region: RecipeMac
pub struct RecipeMac;

impl RecipeMac {
    pub async fn create(db: &Db, utx: &UserCtx, data: RecipePatch) -> Result<Recipe, model::Error> {
        let sql_insert = "INSERT INTO recipes (title, cid) VALUES (?, ?)";

        let title = data.title.unwrap_or_else(|| "Untitled Recipe".to_string());
        let cid = data.cid.unwrap_or(0); // Defaulting cid to 0 for now

        let result = sqlx::query(sql_insert)
            .bind(title)
            .bind(cid)
            .execute(db)
            .await?;

        let last_insert_id = result.last_insert_id();

        let sql_select = "SELECT * FROM recipes WHERE id = ?";
        let recipe = sqlx::query_as::<_, Recipe>(sql_select)
            .bind(last_insert_id)
            .fetch_one(db)
            .await?;

        Ok(recipe)
    }

    pub async fn get(db: &Db, _utx: &UserCtx, id: i64) -> Result<Recipe, model::Error> {
        let sql = "SELECT * FROM recipes WHERE id = ?";

        let result = sqlx::query_as::<_, Recipe>(sql)
            .bind(id)
            .fetch_one(db)
            .await;

        handle_fetch_one_result(result, "recipes", id)
    }

    pub async fn update(
        db: &Db,
        _utx: &UserCtx,
        id: i64,
        data: RecipePatch,
    ) -> Result<Recipe, model::Error> {
        let sql = "UPDATE recipes SET title = ?, cid = ? WHERE id = ?";

        let title = data.title.unwrap_or_else(|| "Untitled Recipe".to_string());
        let cid = data.cid.unwrap_or(0);

        // Perform the update query
        sqlx::query(sql)
            .bind(title)
            .bind(cid)
            .bind(id)
            .execute(db)
            .await?;

        // Return the updated recipe by fetching it again
        let result = sqlx::query_as::<_, Recipe>("SELECT * FROM recipes WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await;

        handle_fetch_one_result(result, "recipes", id)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Recipe>, model::Error> {
        let sql = "SELECT * FROM recipes ORDER BY id DESC";

        let recipes = sqlx::query_as(&sql).fetch_all(db).await?;

        Ok(recipes)
    }

    pub async fn delete(db: &Db, _utx: &UserCtx, id: i64) -> Result<Recipe, model::Error> {
        let sql_select = "SELECT * FROM recipes WHERE id = ?";
        let recipe = sqlx::query_as::<_, Recipe>(sql_select)
            .bind(id)
            .fetch_one(db)
            .await?;

        let sql_delete = "DELETE FROM recipes WHERE id = ?";
        sqlx::query(sql_delete).bind(id).execute(db).await?;

        Ok(recipe)
    }
}
// endregion: RecipeMac

// region: Utils
fn handle_fetch_one_result(
    result: Result<Recipe, sqlx::Error>,
    typ: &'static str,
    id: i64,
) -> Result<Recipe, model::Error> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => model::Error::EntityNotFound("recipes", id.to_string()),
        other => model::Error::SqlxError(other),
    })
}
//endregion: Utils

#[cfg(test)]
#[path = "../_tests/model_recipe.rs"]
mod tests;
