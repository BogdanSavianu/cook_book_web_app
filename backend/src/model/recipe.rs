use crate::{
    model::recipe_ingredient,
    model::{self, db::Db},
    security::UserCtx,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::mysql;
use warp::filters::ws::ws;

use super::recipe_ingredient::{RecipeIngredientMac, RecipeIngredientPatch};

// region: Recipe Types
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub cid: i64,
    pub ctime: DateTime<Utc>,
    pub mtime: DateTime<Utc>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RecipePatch {
    pub recipe_patch: RecipePatchInner,
    pub ingredients: Option<Vec<RecipeIngredientPatch>>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct RecipePatchInner {
    pub title: Option<String>,
    pub cid: Option<i64>,
}
// endregion: Recipe Types

// region: RecipeMac
pub struct RecipeMac;

impl RecipeMac {
    pub async fn create(
        db: &Db,
        utx: &UserCtx,
        data: RecipePatch,
    ) -> Result<(Recipe, Vec<RecipeIngredientMac>), model::Error> {
        let sql_insert = "INSERT INTO recipes (title, cid) VALUES (?, ?)";

        let title = data
            .recipe_patch
            .title
            .unwrap_or_else(|| "Untitled Recipe".to_string());
        let cid = data.recipe_patch.cid.unwrap_or(0);

        let result = sqlx::query(sql_insert)
            .bind(title)
            .bind(cid)
            .execute(db)
            .await?;

        let last_insert_id = result.last_insert_id();

        // Insert recipe ingredients
        if let Some(ingredients) = data.ingredients {
            for ingredient in ingredients {
                sqlx::query(
                    "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, ingredient_name, quantity) VALUES (?, ?, ?, ?)",
                )
                .bind(last_insert_id)
                .bind(ingredient.ingredient_id)
                .bind(ingredient.ingredient_name)
                .bind(ingredient.quantity)
                .execute(db)
                .await?;
            }
        }

        // Fetch the recipe
        let sql_select_recipe = "SELECT * FROM recipes WHERE id = ?";
        let recipe = sqlx::query_as::<_, Recipe>(sql_select_recipe)
            .bind(last_insert_id)
            .fetch_one(db)
            .await?;

        // Fetch the ingredients
        let ingredients = RecipeIngredientMac::list_by_recipe(db, last_insert_id as i64).await?;

        Ok((recipe, ingredients))
    }

    pub async fn get(
        db: &Db,
        _utx: &UserCtx,
        id: i64,
    ) -> Result<(Recipe, Vec<RecipeIngredientMac>), model::Error> {
        // Fetch the recipe
        let sql_recipe = "SELECT * FROM recipes WHERE id = ?";
        let recipe = sqlx::query_as::<_, Recipe>(sql_recipe)
            .bind(id)
            .fetch_one(db)
            .await
            .map_err(|sqlx_error| match sqlx_error {
                sqlx::Error::RowNotFound => model::Error::EntityNotFound("recipes", id.to_string()),
                other => model::Error::SqlxError(other),
            })?;

        // Fetch the ingredients associated with the recipe
        let ingredients = RecipeIngredientMac::list_by_recipe(db, id).await?;

        Ok((recipe, ingredients))
    }

    pub async fn update(
        db: &Db,
        _utx: &UserCtx,
        id: i64,
        data: RecipePatch,
    ) -> Result<(Recipe, Vec<RecipeIngredientMac>), model::Error> {
        let sql = "UPDATE recipes SET title = ?, cid = ? WHERE id = ?";

        let title = data
            .recipe_patch
            .title
            .unwrap_or_else(|| "Untitled Recipe".to_string());
        let cid = data.recipe_patch.cid.unwrap_or(0);

        // Perform the update query
        sqlx::query(sql)
            .bind(title)
            .bind(cid)
            .bind(id)
            .execute(db)
            .await?;

        // Update recipe ingredients
        if let Some(ingredients) = data.ingredients {
            // First delete existing ingredients
            sqlx::query("DELETE FROM recipe_ingredients WHERE recipe_id = ?")
                .bind(id)
                .execute(db)
                .await?;

            // Insert updated ingredients
            for ingredient in ingredients {
                sqlx::query(
                    "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, ingredient_name, quantity) VALUES (?, ?, ?, ?)",
                )
                .bind(id)
                .bind(ingredient.ingredient_id)
                .bind(ingredient.ingredient_name)
                .bind(ingredient.quantity)
                .execute(db)
                .await?;
            }
        }

        // Return the updated recipe and its ingredients
        let sql_select_recipe = "SELECT * FROM recipes WHERE id = ?";
        let recipe = sqlx::query_as::<_, Recipe>(sql_select_recipe)
            .bind(id)
            .fetch_one(db)
            .await?;

        let ingredients = RecipeIngredientMac::list_by_recipe(db, id).await?;

        Ok((recipe, ingredients))
    }

    pub async fn list(
        db: &Db,
        _utx: &UserCtx,
    ) -> Result<Vec<(Recipe, Vec<RecipeIngredientMac>)>, model::Error> {
        let sql = "SELECT * FROM recipes ORDER BY id DESC";

        let recipes = sqlx::query_as::<_, Recipe>(sql).fetch_all(db).await?;

        // Fetch ingredients for each recipe
        let mut result = Vec::new();
        for recipe in recipes {
            let ingredients = RecipeIngredientMac::list_by_recipe(db, recipe.id).await?;
            result.push((recipe, ingredients));
        }

        Ok(result)
    }

    pub async fn delete(
        db: &Db,
        _utx: &UserCtx,
        id: i64,
    ) -> Result<(Recipe, Vec<RecipeIngredientMac>), model::Error> {
        // Fetch the recipe
        let sql_select = "SELECT * FROM recipes WHERE id = ?";
        let recipe = sqlx::query_as::<_, Recipe>(sql_select)
            .bind(id)
            .fetch_one(db)
            .await?;

        // Fetch the ingredients
        let ingredients = RecipeIngredientMac::list_by_recipe(db, id).await?;

        // Delete the recipe
        let sql_delete = "DELETE FROM recipes WHERE id = ?";
        sqlx::query(sql_delete).bind(id).execute(db).await?;

        Ok((recipe, ingredients))
    }
}
// endregion: RecipeMac
#[cfg(test)]
#[path = "../_tests/model_recipe.rs"]
mod tests;
