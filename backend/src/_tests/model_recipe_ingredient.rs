use crate::{
    model::recipe_ingredient::RecipeIngredientPatch,
    model::{
        db::init_db, ingredient::IngredientMac, recipe::RecipeMac,
        recipe_ingredient::RecipeIngredientMac, Error,
    },
    security::utx_from_token,
};

#[tokio::test]
async fn model_recipe_ingredient_create() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    // Fetch the existing "tomato soup" recipe and "tomatoes" ingredient
    let recipe = RecipeMac::get(&db, &utx, 1000).await?; // Assuming "tomato soup" has id 1000
    let ingredient = IngredientMac::get(&db, &utx, 1000).await?; // Assuming "tomatoes" has id 1000

    // Now create a recipe-ingredient entry with valid recipe_id and ingredient_id
    let data_fx = RecipeIngredientPatch {
        recipe_id: Some(recipe.id),
        ingredient_id: Some(ingredient.id),
        quantity: Some("3 tomatoes".to_string()),
        ..Default::default()
    };

    // -- ACTION
    let recipe_ingredient_created = RecipeIngredientMac::create(&db, &utx, data_fx.clone()).await?;

    // -- CHECK
    assert_eq!(
        data_fx.recipe_id.unwrap(),
        recipe_ingredient_created.recipe_id
    );
    assert_eq!(
        data_fx.ingredient_id.unwrap(),
        recipe_ingredient_created.ingredient_id
    );
    assert_eq!(
        data_fx.quantity.unwrap(),
        recipe_ingredient_created.quantity
    );

    Ok(())
}

#[tokio::test]
async fn model_recipe_ingredient_get() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    // Fetch the existing "tomato soup" recipe and "tomatoes" ingredient
    let recipe = RecipeMac::get(&db, &utx, 1000).await?;
    let ingredient = IngredientMac::get(&db, &utx, 1000).await?;

    // Create a recipe-ingredient entry
    let data_fx = RecipeIngredientPatch {
        recipe_id: Some(recipe.id),
        ingredient_id: Some(ingredient.id),
        quantity: Some("3 tomatoes".to_string()),
        ..Default::default()
    };
    let recipe_ingredient_created = RecipeIngredientMac::create(&db, &utx, data_fx.clone()).await?;

    // -- ACTION
    let recipe_ingredient = RecipeIngredientMac::get(
        &db,
        &utx,
        recipe_ingredient_created.recipe_id,
        recipe_ingredient_created.ingredient_id,
    )
    .await?;

    // -- CHECK
    assert_eq!(
        recipe_ingredient_created.recipe_id,
        recipe_ingredient.recipe_id
    );
    assert_eq!(
        recipe_ingredient_created.ingredient_id,
        recipe_ingredient.ingredient_id
    );
    assert_eq!(
        recipe_ingredient_created.quantity,
        recipe_ingredient.quantity
    );

    Ok(())
}

#[tokio::test]
async fn model_recipe_ingredient_get_wrong_id() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    // -- ACTION
    let result = RecipeIngredientMac::get(&db, &utx, 9999, 9999).await;

    // -- CHECK
    match result {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(Error::EntityNotFound(typ, id)) => {
            assert_eq!("recipe_ingredients", typ);
            assert_eq!(9999.to_string(), id);
        }
        other_error => assert!(false, "Wrong Error: {:?}", other_error),
    }

    Ok(())
}

#[tokio::test]
async fn model_recipe_ingredient_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    // Fetch the existing "tomato soup" recipe and "tomatoes" ingredient
    let recipe = RecipeMac::get(&db, &utx, 1000).await?;
    let ingredient = IngredientMac::get(&db, &utx, 1000).await?;

    // Create a recipe-ingredient entry
    let data_fx = RecipeIngredientPatch {
        recipe_id: Some(recipe.id),
        ingredient_id: Some(ingredient.id),
        quantity: Some("3 tomatoes".to_string()),
        ..Default::default()
    };
    let recipe_ingredient = RecipeIngredientMac::create(&db, &utx, data_fx.clone()).await?;

    // Update quantity
    let updated_data_fx = RecipeIngredientPatch {
        quantity: Some("4 tomatoes".to_string()),
        ..Default::default()
    };

    // -- ACTION
    let recipe_ingredient_updated = RecipeIngredientMac::update(
        &db,
        &utx,
        recipe_ingredient.recipe_id,
        recipe_ingredient.ingredient_id,
        updated_data_fx.clone(),
    )
    .await?;

    // -- CHECK
    assert_eq!(
        recipe_ingredient.recipe_id,
        recipe_ingredient_updated.recipe_id
    );
    assert_eq!(
        recipe_ingredient.ingredient_id,
        recipe_ingredient_updated.ingredient_id
    );
    assert_eq!(
        updated_data_fx.quantity.unwrap(),
        recipe_ingredient_updated.quantity
    );

    Ok(())
}

#[tokio::test]
async fn model_recipe_ingredient_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    // Fetch the existing "tomato soup" recipe and "tomatoes" ingredient
    let recipe = RecipeMac::get(&db, &utx, 1000).await?;
    let ingredient = IngredientMac::get(&db, &utx, 1000).await?;

    // Create a recipe-ingredient entry
    let data_fx = RecipeIngredientPatch {
        recipe_id: Some(recipe.id),
        ingredient_id: Some(ingredient.id),
        quantity: Some("3 tomatoes".to_string()),
        ..Default::default()
    };
    RecipeIngredientMac::create(&db, &utx, data_fx.clone()).await?;

    // -- ACTION
    let recipe_ingredients = RecipeIngredientMac::list(&db, &utx).await?;

    // -- CHECK
    assert!(recipe_ingredients.len() > 0);
    assert_eq!(recipe.id, recipe_ingredients[0].recipe_id);
    assert_eq!(ingredient.id, recipe_ingredients[0].ingredient_id);

    Ok(())
}

#[tokio::test]
async fn model_recipe_ingredient_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    // Fetch the existing "tomato soup" recipe and "tomatoes" ingredient
    let recipe = RecipeMac::get(&db, &utx, 1000).await?;
    let ingredient = IngredientMac::get(&db, &utx, 1000).await?;

    // Create a recipe-ingredient entry
    let data_fx = RecipeIngredientPatch {
        recipe_id: Some(recipe.id),
        ingredient_id: Some(ingredient.id),
        quantity: Some("3 tomatoes".to_string()),
        ..Default::default()
    };
    let recipe_ingredient = RecipeIngredientMac::create(&db, &utx, data_fx.clone()).await?;

    // -- ACTION
    let recipe_ingredient_deleted = RecipeIngredientMac::delete(
        &db,
        &utx,
        recipe_ingredient.recipe_id,
        recipe_ingredient.ingredient_id,
    )
    .await?;

    // -- CHECK - deleted item
    assert_eq!(
        recipe_ingredient.recipe_id,
        recipe_ingredient_deleted.recipe_id
    );
    assert_eq!(
        recipe_ingredient.ingredient_id,
        recipe_ingredient_deleted.ingredient_id
    );

    Ok(())
}
