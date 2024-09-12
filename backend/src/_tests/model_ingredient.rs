use crate::{
    model::{self, db::init_db},
    security::utx_from_token,
};

use super::{IngredientMac, IngredientPatch};

#[tokio::test]
async fn model_ingredient_create() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    let data_fx = IngredientPatch {
        name: Some("test - model_ingredient_create 1".to_string()),
        quantity: Some("test - model_ingredient_quantity 1".to_string()),
        ..Default::default()
    };

    // -- ACTION
    let ingredient_created = IngredientMac::create(&db, &utx, data_fx.clone()).await?;

    // -- CHECK
    assert!(ingredient_created.id >= 1000, "ID should be >= 1000");
    assert_eq!(data_fx.name.unwrap(), ingredient_created.name);

    Ok(())
}

#[tokio::test]
async fn model_ingredient_get() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let ingredient = IngredientMac::get(&db, &utx, 1000).await?;

    // -- CHECK
    assert_eq!(1000, ingredient.id);
    assert_eq!("tomatoes", ingredient.name);

    Ok(())
}

#[tokio::test]
async fn model_ingredient_get_wrong_id() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let result = IngredientMac::get(&db, &utx, 99).await;

    // -- CHECK
    match result {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::EntityNotFound(typ, id)) => {
            assert_eq!("ingredients", typ);
            assert_eq!(99.to_string(), id);
        }
        other_error => assert!(false, "Wrong Error: {:?}", other_error),
    }

    Ok(())
}

#[tokio::test]
async fn model_ingredient_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    let data_fx = IngredientPatch {
        name: Some("test - model_ingredient_update_ok 1".to_string()),
        quantity: Some("test - model_ingredient_update_ok 1".to_string()),
        ..Default::default()
    };
    let ingredient_fx = IngredientMac::create(&db, &utx, data_fx.clone()).await?;
    let update_data_fx = IngredientPatch {
        name: Some("test - model_ingredient_update_ok 2".to_string()),
        quantity: Some("test - model_ingredient_update_ok 2".to_string()),
        ..Default::default()
    };

    // -- ACTION
    let ingredient_updated =
        IngredientMac::update(&db, &utx, ingredient_fx.id, update_data_fx.clone()).await?;

    // -- CHECK
    let ingredients = IngredientMac::list(&db, &utx).await?;
    assert_eq!(2, ingredients.len());
    assert_eq!(ingredient_fx.id, ingredient_updated.id);
    assert_eq!(update_data_fx.name.unwrap(), ingredient_updated.name);

    Ok(())
}

#[tokio::test]
async fn model_ingredient_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let ingredients = IngredientMac::list(&db, &utx).await?;

    // -- CHECK
    assert_eq!(1, ingredients.len());
    // ingredient 1000
    assert_eq!(1000, ingredients[0].id);
    assert_eq!("tomatoes", ingredients[0].name);

    Ok(())
}

#[tokio::test]
async fn model_ingredient_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let ingredient = IngredientMac::delete(&db, &utx, 1000).await?;

    // -- CHECK - deleted item
    assert_eq!(1000, ingredient.id);
    assert_eq!("tomatoes", ingredient.name);

    // -- CHECK - list
    let ingredients = IngredientMac::list(&db, &utx).await?;
    assert_eq!(0, ingredients.len());

    Ok(())
}
