use super::{RecipeIngredientPatch, RecipeMac, RecipePatch, RecipePatchInner};
use crate::{
    model::{self, db::init_db},
    security::utx_from_token,
};

#[tokio::test]
async fn model_recipe_create() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    let data_fx = RecipePatch {
        recipe_patch: RecipePatchInner {
            title: Some("test - model_recipe_create 1".to_string()),
            cid: Some(123),
            ..Default::default()
        },
        ingredients: Some(vec![RecipeIngredientPatch {
            ingredient_id: 1000,
            ingredient_name: "tomatoes".to_string(),
            quantity: "2 cups".to_string(),
        }]),
    };

    // -- ACTION
    let recipe_created = RecipeMac::create(&db, &utx, data_fx.clone()).await?;

    // -- CHECK
    assert!(recipe_created.0.id >= 1000, "ID should be >= 1000");
    assert_eq!(data_fx.recipe_patch.title.unwrap(), recipe_created.0.title);

    Ok(())
}

#[tokio::test]
async fn model_recipe_get() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    let data_fx = RecipePatch {
        recipe_patch: RecipePatchInner {
            title: Some("tomato soup".to_string()),
            cid: Some(123),
            ..Default::default()
        },
        ingredients: Some(vec![RecipeIngredientPatch {
            ingredient_id: 1000,
            ingredient_name: "tomatoes".to_string(),
            quantity: "2 cups".to_string(),
        }]),
    };

    let recipe_created = RecipeMac::create(&db, &utx, data_fx.clone()).await?;

    // -- ACTION
    let (recipe, ingredients) = RecipeMac::get(&db, &utx, 1001).await?;

    // -- CHECK
    assert_eq!(1001, recipe.id);
    assert_eq!("tomato soup", recipe.title);
    assert_eq!(1, ingredients.len());
    assert_eq!(1000, ingredients[0].ingredient_id);
    assert_eq!("2 cups", ingredients[0].quantity);

    Ok(())
}

#[tokio::test]
async fn model_recipe_get_wrong_id() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let result = RecipeMac::get(&db, &utx, 99).await;

    // -- CHECK
    match result {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::EntityNotFound(typ, id)) => {
            assert_eq!("recipes", typ);
            assert_eq!(99.to_string(), id);
        }
        other_error => assert!(false, "Wrong Error: {:?}", other_error),
    }

    Ok(())
}

#[tokio::test]
async fn model_recipe_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    let data_fx = RecipePatch {
        recipe_patch: RecipePatchInner {
            title: Some("test - model_recipe_update_ok 1".to_string()),
            cid: Some(123),
            ..Default::default()
        },
        ingredients: Some(vec![RecipeIngredientPatch {
            ingredient_id: 1000,
            ingredient_name: "tomatoes".to_string(),
            quantity: "2 cups".to_string(),
        }]),
    };
    let recipe_fx = RecipeMac::create(&db, &utx, data_fx.clone()).await?;
    let update_data_fx = RecipePatch {
        recipe_patch: RecipePatchInner {
            title: Some("test - model_recipe_update_ok 2".to_string()),
            ..Default::default()
        },
        ingredients: Some(vec![RecipeIngredientPatch {
            ingredient_id: 1000,
            ingredient_name: "tomatoes".to_string(),
            quantity: "3 tbsp".to_string(),
        }]),
    };

    // -- ACTION
    let recipe_updated =
        RecipeMac::update(&db, &utx, recipe_fx.0.id, update_data_fx.clone()).await?;

    // -- CHECK
    let recipes = RecipeMac::list(&db, &utx).await?;
    assert_eq!(2, recipes.len());
    assert_eq!(recipe_fx.0.id, recipe_updated.0.id);
    assert_eq!(
        update_data_fx.recipe_patch.title.unwrap(),
        recipe_updated.0.title
    );

    Ok(())
}

#[tokio::test]
async fn model_recipe_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let recipes = RecipeMac::list(&db, &utx).await?;

    // -- CHECK
    assert_eq!(1, recipes.len());
    assert_eq!(1000, recipes[0].0.id);
    assert_eq!("spaghetti", recipes[0].0.title);

    Ok(())
}

#[tokio::test]
async fn model_recipe_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let recipe = RecipeMac::delete(&db, &utx, 1000).await?;

    // -- CHECK - deleted item
    assert_eq!(1000, recipe.0.id);
    assert_eq!("spaghetti", recipe.0.title);

    // -- CHECK - list
    let recipes = RecipeMac::list(&db, &utx).await?;
    assert_eq!(0, recipes.len());

    Ok(())
}
