#[macro_use]
extern crate lazy_static;
use log::info;
use settings::logging;
use warp::Rejection;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type DB = mongodb::Database;

mod app;
mod data;
mod db;
mod error;
mod routes;
mod settings;
mod web;

lazy_static! {
    // Globally accessible configuration
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[tokio::main]
async fn main() -> Result<()> {
    logging::init(&CONFIG.log.level);

    let (db, _client) = db::init().await?;

    info!("Started on port 8080");
    let routes = routes::router(db);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}
