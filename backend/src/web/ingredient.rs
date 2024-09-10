use crate::model::{Db, IngredientMac, IngredientPatch};
use crate::security::{utx_from_token, UserCtx};
use serde::Serialize;
use serde_json::json;
use std::{convert::Infallible, sync::Arc};
use warp::reply::Json;
use warp::{Filter, Rejection};

use super::filter_auth::do_auth;
use super::filter_utils::with_db;

pub fn ingredient_rest_filters(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let ingredients_path = warp::path(base_path).and(warp::path("ingredients"));
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    /// LIST ingredients 'GET ingredients/'
    let list = ingredients_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(ingredient_list);

    /// GET ingredient 'GET /ingredients/1000'
    let get = ingredients_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(ingredient_get);

    /// CREATE ingredient 'POST /ingredients with body IngredientPatch'
    let create = ingredients_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(ingredient_create);

    /// UPDATE ingredient 'PATCH /ingredients/1000 with body IngredientPatch'
    let update = ingredients_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(ingredient_update);

    /// DELETE ingredient 'DELETE /ingredients/1000'
    let delete = ingredients_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(ingredient_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn ingredient_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    // FIXME: Add proper error handling
    let ingredients = IngredientMac::list(&db, &utx).await?;
    json_response(ingredients)
}

async fn ingredient_get(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let ingredient = IngredientMac::get(&db, &utx, id).await?;
    json_response(ingredient)
}

async fn ingredient_create(
    db: Arc<Db>,
    utx: UserCtx,
    patch: IngredientPatch,
) -> Result<Json, warp::Rejection> {
    let ingredient = IngredientMac::create(&db, &utx, patch).await?;
    json_response(ingredient)
}

async fn ingredient_update(
    db: Arc<Db>,
    utx: UserCtx,
    id: i64,
    patch: IngredientPatch,
) -> Result<Json, warp::Rejection> {
    let ingredient = IngredientMac::update(&db, &utx, id, patch).await?;
    json_response(ingredient)
}

async fn ingredient_delete(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let ingredient = IngredientMac::delete(&db, &utx, id).await?;
    json_response(ingredient)
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}
// region: Test
#[cfg(test)]
#[path = "../_tests/web_ingredient.rs"]
mod tests;
// endregion: Test
