use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{fs, path::PathBuf, time::Duration};

const HOST: &str = "localhost:3306";
const ROOT_DB: &str = "cookbook";
const ROOT_USER: &str = "root";
const ROOT_PWD: &str = "macmacmac";

// app db
const APP_DB: &str = "cookbook";
const APP_USER: &str = "bogdan";
const APP_PWD: &str = "macmacmac";
const APP_MAX_CON: u32 = 5;

// sql files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub type Db = Pool<MySql>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    // -- Create the db with ROOT (dev only)
    {
        let root_db = new_db_pool(HOST, ROOT_DB, ROOT_USER, ROOT_PWD, 1).await?;
        pexec(&root_db, SQL_RECREATE).await?;
    }

    // -- Run the app sql files
    let app_db = new_db_pool(HOST, APP_DB, APP_USER, APP_PWD, APP_MAX_CON).await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    paths.sort();
    // execute each file
    for path in paths {
        if let Some(path) = path.to_str() {
            // only .sql and not recreate
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&app_db, &path).await?;
            }
        }
    }

    //returning the app db
    new_db_pool(HOST, APP_DB, APP_USER, APP_PWD, APP_MAX_CON).await
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("ERROR reading {} (cause: {:?})", file, ex);
        ex
    })?;

    let sqls: Vec<&str> = content
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(ex) => println!("WARNING - pexec - Sql file '{}' FAILED cause: {}", file, ex),
        }
    }

    Ok(())
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let con_string = format!("mysql://{}:{}@{}/{}", user, pwd, host, db);

    MySqlPoolOptions::new()
        .max_connections(max_con)
        .connect_timeout(Duration::from_millis(500))
        .connect(&con_string)
        .await
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
