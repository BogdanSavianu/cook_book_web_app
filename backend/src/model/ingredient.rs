use serde::{Deserialize, Serialize};
use warp::filters::ws::ws;

use super::db::{self, Db};
use crate::{model, security::UserCtx};
use sqlb::{HasFields, Raw, SqlBuilder};
use sqlx::mysql;

// region: Ingredient Types
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Clone, Deserialize)]
pub struct IngredientPatch {
    pub name: Option<String>,
}
// endregion: Ingredient Types

// region: IngredientMac
pub struct IngredientMac;

impl IngredientMac {
    pub async fn create(
        db: &Db,
        utx: &UserCtx,
        data: IngredientPatch,
    ) -> Result<Ingredient, model::Error> {
        let sql_insert = "INSERT INTO ingredients (name) VALUES (?)";

        let result = sqlx::query(sql_insert)
            .bind(data.name.unwrap_or_else(|| "untitled".to_string()))
            .execute(db)
            .await?;

        let last_insert_id = result.last_insert_id();

        let sql_select = "SELECT id, name FROM ingredients WHERE id = ?";
        let ingredient = sqlx::query_as::<_, Ingredient>(sql_select)
            .bind(last_insert_id)
            .fetch_one(db)
            .await?;

        Ok(ingredient)
    }

    pub async fn get(db: &Db, _utx: &UserCtx, id: i64) -> Result<Ingredient, model::Error> {
        let sql = "SELECT * from ingredients WHERE id = (?)";

        let result = sqlx::query_as::<_, Ingredient>(sql)
            .bind(id)
            .fetch_one(db)
            .await;

        handle_fetch_one_result(result, "ingredient", id)
    }

    pub async fn update(
        db: &Db,
        _utx: &UserCtx,
        id: i64,
        data: IngredientPatch,
    ) -> Result<Ingredient, model::Error> {
        let sql = "UPDATE ingredients SET name = ? WHERE id = ?";

        let name = data.name.unwrap_or_else(|| "untitled".to_string());

        // Perform the update query
        sqlx::query(sql).bind(name).bind(id).execute(db).await?;

        // Return the updated ingredient by fetching it again
        let result = sqlx::query_as::<_, Ingredient>("SELECT * FROM ingredients WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await;

        handle_fetch_one_result(result, "ingredients", id)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Ingredient>, model::Error> {
        let sql = "SELECT * FROM ingredients ORDER BY id DESC";

        // build the sqlx-query
        let query = sqlx::query_as(&sql);
        //execute the query
        let ingredients = query.fetch_all(db).await?;

        Ok(ingredients)
    }

    pub async fn delete(db: &Db, _utx: &UserCtx, id: i64) -> Result<Ingredient, model::Error> {
        // Fetch the ingredient before deleting it
        let sql_select = "SELECT * FROM ingredients WHERE id = ?";
        let ingredient = sqlx::query_as::<_, Ingredient>(sql_select)
            .bind(id)
            .fetch_one(db)
            .await?;

        // Now delete the ingredient
        let sql_delete = "DELETE FROM ingredients WHERE id = ?";
        sqlx::query(sql_delete).bind(id).execute(db).await?;

        // Return the fetched ingredient as the deleted one
        Ok(ingredient)
    }
}
// endregion: IngredientMac

// region: Utils
fn handle_fetch_one_result(
    result: Result<Ingredient, sqlx::Error>,
    typ: &'static str,
    id: i64,
) -> Result<Ingredient, model::Error> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => model::Error::EntityNotFound("ingredients", id.to_string()),
        other => model::Error::SqlxError(other),
    })
}
//endregion: Utils

#[cfg(test)]
#[path = "../_tests/model_ingredient.rs"]
mod tests;
