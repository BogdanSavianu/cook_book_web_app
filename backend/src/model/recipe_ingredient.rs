use super::db::{self, Db};
use crate::{model, security::UserCtx};
use chrono::{DateTime, Utc};
use sqlx::mysql;

// region: RecipeIngredient Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct RecipeIngredient {
    pub recipe_id: i64,
    pub ingredient_id: i64,
    pub quantity: String,
    pub cid: i64,             // Creator ID
    pub ctime: DateTime<Utc>, // Creation time
    pub mtime: DateTime<Utc>, // Last modification time
}

#[derive(Default, Clone)]
pub struct RecipeIngredientPatch {
    pub recipe_id: Option<i64>,
    pub ingredient_id: Option<i64>,
    pub quantity: Option<String>,
}
// endregion: RecipeIngredient Types

// region: RecipeIngredientMac
pub struct RecipeIngredientMac;

impl RecipeIngredientMac {
    pub async fn create(
        db: &Db,
        utx: &UserCtx,
        data: RecipeIngredientPatch,
    ) -> Result<RecipeIngredient, model::Error> {
        let sql_insert = r#"
            INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, cid)
            VALUES (?, ?, ?, ?)
        "#;

        let recipe_id = data.recipe_id.unwrap();
        let ingredient_id = data.ingredient_id.unwrap();
        let quantity = data.quantity.unwrap_or_else(|| "1 unit".to_string());
        let cid = utx.user_id;

        sqlx::query(sql_insert)
            .bind(recipe_id)
            .bind(ingredient_id)
            .bind(quantity)
            .bind(cid)
            .execute(db)
            .await?;

        let sql_select = r#"
            SELECT recipe_id, ingredient_id, quantity, cid, ctime, mtime 
            FROM recipe_ingredients 
            WHERE recipe_id = ? AND ingredient_id = ?
        "#;

        let recipe_ingredient = sqlx::query_as::<_, RecipeIngredient>(sql_select)
            .bind(recipe_id)
            .bind(ingredient_id)
            .fetch_one(db)
            .await?;

        Ok(recipe_ingredient)
    }

    pub async fn get(
        db: &Db,
        _utx: &UserCtx,
        recipe_id: i64,
        ingredient_id: i64,
    ) -> Result<RecipeIngredient, model::Error> {
        let sql = r#"
            SELECT recipe_id, ingredient_id, quantity, cid, ctime, mtime
            FROM recipe_ingredients
            WHERE recipe_id = ?
        "#;

        let result = sqlx::query_as::<_, RecipeIngredient>(sql)
            .bind(recipe_id)
            .fetch_one(db)
            .await;

        handle_fetch_one_result(result, recipe_id, ingredient_id)
    }

    pub async fn update(
        db: &Db,
        _utx: &UserCtx,
        recipe_id: i64,
        ingredient_id: i64,
        data: RecipeIngredientPatch,
    ) -> Result<RecipeIngredient, model::Error> {
        let sql = r#"
            UPDATE recipe_ingredients
            SET quantity = ?
            WHERE recipe_id = ? AND ingredient_id = ?
        "#;

        let quantity = data.quantity.unwrap_or_else(|| "1 unit".to_string());

        sqlx::query(sql)
            .bind(quantity)
            .bind(recipe_id)
            .bind(ingredient_id)
            .execute(db)
            .await?;

        let result = sqlx::query_as::<_, RecipeIngredient>(
            r#"
            SELECT recipe_id, ingredient_id, quantity, cid, ctime, mtime
            FROM recipe_ingredients
            WHERE recipe_id = ? AND ingredient_id = ?
            "#,
        )
        .bind(recipe_id)
        .bind(ingredient_id)
        .fetch_one(db)
        .await;

        handle_fetch_one_result(result, recipe_id, ingredient_id)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<RecipeIngredient>, model::Error> {
        let sql = r#"
            SELECT recipe_id, ingredient_id, quantity, cid, ctime, mtime 
            FROM recipe_ingredients 
            ORDER BY recipe_id DESC
        "#;

        let recipe_ingredients = sqlx::query_as::<_, RecipeIngredient>(&sql)
            .fetch_all(db)
            .await?;

        Ok(recipe_ingredients)
    }

    pub async fn delete(
        db: &Db,
        _utx: &UserCtx,
        recipe_id: i64,
        ingredient_id: i64,
    ) -> Result<RecipeIngredient, model::Error> {
        let sql_select = r#"
            SELECT recipe_id, ingredient_id, quantity, cid, ctime, mtime
            FROM recipe_ingredients
            WHERE recipe_id = ? AND ingredient_id = ?
        "#;

        let recipe_ingredient = sqlx::query_as::<_, RecipeIngredient>(sql_select)
            .bind(recipe_id)
            .bind(ingredient_id)
            .fetch_one(db)
            .await?;

        let sql_delete = "DELETE FROM recipe_ingredients WHERE recipe_id = ? AND ingredient_id = ?";
        sqlx::query(sql_delete)
            .bind(recipe_id)
            .bind(ingredient_id)
            .execute(db)
            .await?;

        Ok(recipe_ingredient)
    }
}
// endregion: RecipeIngredientMac

// region: Utils
fn handle_fetch_one_result(
    result: Result<RecipeIngredient, sqlx::Error>,
    recipe_id: i64,
    ingredient_id: i64,
) -> Result<RecipeIngredient, model::Error> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => {
            model::Error::EntityNotFound("recipe_ingredients", recipe_id.to_string())
        }
        other => model::Error::SqlxError(other),
    })
}
//endregion: Utils

#[cfg(test)]
#[path = "../_tests/model_recipe_ingredient.rs"]
mod tests;
