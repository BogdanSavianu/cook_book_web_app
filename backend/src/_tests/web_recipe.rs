use crate::model::{init_db, Db, RecipeIngredientPatch, RecipeMac, RecipePatch, RecipePatchInner};
use crate::security::UserCtx;
use crate::web::handle_rejection;
use crate::web::recipe::recipe_rest_filters;
use anyhow::Result;
use serde_json::json;
use std::sync::Arc;
use warp::Filter;

#[tokio::test]
async fn web_recipe_create() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    let recipe_patch = RecipePatch {
        recipe_patch: RecipePatchInner {
            title: Some("New Recipe".to_string()),
            cid: Some(123),
            ..Default::default()
        },
        ingredients: Some(vec![RecipeIngredientPatch {
            ingredient_id: 1000,
            ingredient_name: "tomatoes".to_string(),
            quantity: "1 cup".to_string(),
        }]),
    };

    let response = warp::test::request()
        .method("POST")
        .path("/api/recipes")
        .header("X-Auth-Token", "123")
        .json(&recipe_patch)
        .reply(&recipe_apis)
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();

    //println!("{:#?}", body);
    //
    // Access the recipe and ingredients
    let recipe = &body["data"][0];
    let ingredients = &body["data"][1];

    assert_eq!(recipe["title"], "New Recipe");
    assert_eq!(ingredients[0]["ingredient_name"], "tomatoes");

    Ok(())
}

#[tokio::test]
async fn web_recipe_get() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    let response = warp::test::request()
        .method("GET")
        .path("/api/recipes/1000")
        .header("X-Auth-Token", "123")
        .reply(&recipe_apis)
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    //println!("{:#?}", body);
    let recipe = &body["data"][0];
    assert_eq!(recipe["id"], 1000);
    Ok(())
}

#[tokio::test]
async fn web_recipe_update() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    let recipe_patch = RecipePatch {
        recipe_patch: RecipePatchInner {
            title: Some("Updated Recipe".to_string()),
            ..Default::default()
        },
        ingredients: Some(vec![RecipeIngredientPatch {
            ingredient_id: 1000,
            ingredient_name: "tomatoes".to_string(),
            quantity: "2 cups".to_string(),
        }]),
    };

    let response = warp::test::request()
        .method("PATCH")
        .path("/api/recipes/1000")
        .header("X-Auth-Token", "123")
        .json(&recipe_patch)
        .reply(&recipe_apis)
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();

    // Access the recipe
    let recipe = &body["data"][0];

    assert_eq!(recipe["title"], "Updated Recipe");
    Ok(())
}

#[tokio::test]
async fn web_recipe_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    let response = warp::test::request()
        .method("GET")
        .path("/api/recipes")
        .header("X-Auth-Token", "123")
        .reply(&recipe_apis)
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();

    // Check if the data field is an array
    let data = &body["data"];
    assert!(data.is_array());

    // Check the first item in the list
    if let Some(first_item) = data.get(0) {
        //println!("{:#?}", first_item);
        assert!(first_item.is_array());
    }
    Ok(())
}

#[tokio::test]
async fn web_recipe_delete() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    let response = warp::test::request()
        .method("DELETE")
        .path("/api/recipes/1000")
        .header("X-Auth-Token", "123")
        .reply(&recipe_apis)
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();

    // Access the recipe
    let recipe = &body["data"][0];

    assert_eq!(recipe["id"], 1000);

    Ok(())
}
