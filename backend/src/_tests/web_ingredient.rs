use std::str::from_utf8;
use std::sync::Arc;

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, Value};
use warp::hyper::{body::Bytes, Response};
use warp::reply::Json;
use warp::Filter;

use crate::model::{init_db, Ingredient, IngredientMac};
use crate::security::utx_from_token;
use crate::web::handle_rejection;
use crate::web::ingredient::ingredient_rest_filters;

#[tokio::test]
async fn web_ingredient_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let ingredient_apis = ingredient_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "123")
        .path("/api/ingredients")
        .reply(&ingredient_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let ingredients: Vec<Ingredient> = extract_body_data(resp)?;

    // -- CHECK
    assert_eq!(1, ingredients.len(), "number of ingredients");
    assert_eq!(1000, ingredients[0].id);
    assert_eq!("tomatoes", ingredients[0].name);

    Ok(())
}

#[tokio::test]
async fn web_ingredient_get_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let ingredient_apis = ingredient_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "123")
        .path("/api/ingredients/1000")
        .reply(&ingredient_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let ingredient: Ingredient = extract_body_data(resp)?;

    // -- CHECK - .data (ingredient)
    assert_eq!(1000, ingredient.id);
    assert_eq!("tomatoes", ingredient.name);

    Ok(())
}

#[tokio::test]
async fn web_ingredient_create_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let ingredient_apis = ingredient_rest_filters("api", db.clone()).recover(handle_rejection);

    // new ingredient fixture
    const NAME: &str = "test - web_ingredient_create_ok";
    let body = json!({
        "name": NAME,
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("POST")
        .header("X-Auth-Token", "123")
        .path("/api/ingredients")
        .json(&body)
        .reply(&ingredient_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let ingredient: Ingredient = extract_body_data(resp)?;

    // -- CHECK - .data (ingredient)
    assert!(ingredient.id >= 1000, "ingredient.id should be >= 1000");
    assert_eq!(NAME, ingredient.name);

    Ok(())
}

#[tokio::test]
async fn web_ingredient_update_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let ingredient_apis = ingredient_rest_filters("api", db.clone()).recover(handle_rejection);

    // updated ingredient
    const NAME: &str = "test - ingredient 1000 updated";
    let body = json!({
        "name": NAME
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("PATCH")
        .header("X-Auth-Token", "123")
        .path("/api/ingredients/1000")
        .json(&body)
        .reply(&ingredient_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let ingredient: Ingredient = extract_body_data(resp)?;

    // -- CHECK - .data (ingredient)
    assert_eq!(1000, ingredient.id, "ingredient.id");
    assert_eq!(NAME, ingredient.name);

    Ok(())
}

#[tokio::test]
async fn web_ingredient_delete_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let ingredient_apis = ingredient_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("DELETE")
        .header("X-Auth-Token", "123")
        .path("/api/ingredients/1000")
        .reply(&ingredient_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let ingredient: Ingredient = extract_body_data(resp)?;

    // -- CHECK - .data (ingredients)
    assert_eq!(1000, ingredient.id);
    assert_eq!("tomatoes", ingredient.name);

    // -- CHECK - list .len() should be 0
    let utx = utx_from_token(&db, "123").await?;
    let ingredients = IngredientMac::list(&db, &utx).await?;
    assert_eq!(0, ingredients.len());

    Ok(())
}

// region Web Test Utils
fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where
    for<'de> D: Deserialize<'de>,
{
    // parse the body as serde_json::Value
    let body = from_utf8(resp.body())?;
    let mut body: Value = from_str(body)
        .with_context(|| format!("Cannot parse resp.body to JSON. Resp body: '{}'", body))?;

    // extract the data
    let data = body["data"].take();

    // deserialize the data to D
    let data: D = from_value(data)?;

    Ok(data)
}
// endregion: Web Test Utils
