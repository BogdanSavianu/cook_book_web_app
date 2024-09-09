use thiserror::Error as ThisError;

mod db;
mod ingredient;
mod recipe;
mod recipe_ingredient;

// re-export
pub use db::{init_db, Db};
pub use ingredient::{Ingredient, IngredientMac, IngredientPatch};

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}]")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
