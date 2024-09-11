use crate::model::{Db, RecipeMac, RecipePatch};
use crate::security::{utx_from_token, UserCtx};
use serde::Serialize;
use serde_json::json;
use std::{convert::Infallible, sync::Arc};
use warp::reply::Json;
use warp::{Filter, Rejection};

use super::filter_auth::do_auth;
use super::filter_utils::with_db;

pub fn recipe_rest_filters(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let recipes_path = warp::path(base_path).and(warp::path("recipes"));
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    /// LIST recipes 'GET /recipes'
    let list = recipes_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(recipe_list);

    /// GET recipe 'GET /recipes/1000'
    let get = recipes_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(recipe_get);

    /// CREATE recipe 'POST /recipes with body RecipePatch'
    let create = recipes_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(recipe_create);

    /// UPDATE recipe 'PATCH /recipes/1000 with body RecipePatch'
    let update = recipes_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(recipe_update);

    /// DELETE recipe 'DELETE /recipes/1000'
    let delete = recipes_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(recipe_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn recipe_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    // FIXME: Add proper error handling
    let recipes = RecipeMac::list(&db, &utx).await?;
    json_response(recipes)
}

async fn recipe_get(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let (recipe, ingredients) = RecipeMac::get(&db, &utx, id).await?;
    json_response((recipe, ingredients))
}

async fn recipe_create(
    db: Arc<Db>,
    utx: UserCtx,
    patch: RecipePatch,
) -> Result<Json, warp::Rejection> {
    let recipe = RecipeMac::create(&db, &utx, patch).await?;
    json_response(recipe)
}

async fn recipe_update(
    db: Arc<Db>,
    utx: UserCtx,
    id: i64,
    patch: RecipePatch,
) -> Result<Json, warp::Rejection> {
    let recipe = RecipeMac::update(&db, &utx, id, patch).await?;
    json_response(recipe)
}

async fn recipe_delete(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let recipe = RecipeMac::delete(&db, &utx, id).await?;
    json_response(recipe)
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}

// region: Test
#[cfg(test)]
#[path = "../_tests/web_recipe.rs"]
mod tests;
// endregion: Test// region: Test
