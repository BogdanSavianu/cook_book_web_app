use std::str::from_utf8;
use std::sync::Arc;

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, Value};
use warp::hyper::{body::Bytes, Response};
use warp::reply::Json;

use crate::model::{init_db, Ingredient, IngredientMac};
use crate::web::ingredient::ingredient_rest_filters;

#[tokio::test]
async fn web_ingredient_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let ingredient_apis = ingredient_rest_filters("api", db.clone());

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
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
