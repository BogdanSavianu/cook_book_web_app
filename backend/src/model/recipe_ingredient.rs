use crate::model::db::Db;
use crate::security::UserCtx;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::mysql;

// region: Recipe Ingredient Types
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct RecipeIngredientMac {
    pub recipe_id: i64,
    pub ingredient_id: i64,
    pub ingredient_name: String,
    pub quantity: String,
    pub cid: i64,
    pub ctime: DateTime<Utc>,
    pub mtime: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct RecipeIngredientPatch {
    pub ingredient_id: i64,
    pub ingredient_name: String,
    pub quantity: String,
}
// endregion: Recipe Ingredient Types

// region: RecipeIngredientMac
impl RecipeIngredientMac {
    pub async fn list_by_recipe(
        db: &Db,
        recipe_id: i64,
    ) -> Result<Vec<RecipeIngredientMac>, sqlx::Error> {
        let sql = "SELECT * FROM recipe_ingredients WHERE recipe_id = ?";
        let ingredients = sqlx::query_as::<_, RecipeIngredientMac>(sql)
            .bind(recipe_id)
            .fetch_all(db)
            .await?;
        Ok(ingredients)
    }

    pub async fn delete_by_recipe(db: &Db, recipe_id: i64) -> Result<(), sqlx::Error> {
        let sql = "DELETE FROM recipe_ingredients WHERE recipe_id = ?";
        sqlx::query(sql).bind(recipe_id).execute(db).await?;
        Ok(())
    }
} // endregion: RecipeIngredientMac
