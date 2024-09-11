use std::sync::Arc;

use warp::{reject::Rejection, Filter};

use crate::{
    model::Db,
    security::{utx_from_token, UserCtx},
};

use super::{filter_utils::with_db, Error};

const HEADER_XAUTH: &str = "X-Auth-Token";

pub fn do_auth(db: Arc<Db>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
    //warp::any().and_then(|| async { Ok::<UserCtx, Rejection>(utx_from_token("123").await?) })
    warp::any()
        .and(with_db(db))
        .and(warp::header::optional(HEADER_XAUTH))
        .and_then(|db: Arc<Db>, xauth: Option<String>| async move {
            match xauth {
                Some(xauth) => {
                    let utx = utx_from_token(&db, &xauth).await?;
                    Ok::<UserCtx, Rejection>(utx)
                }
                None => Err(Error::FailAuthMissingXAuth.into()),
            }
        })
}
