#[macro_use]
extern crate lazy_static;
use log::info;
use settings::logging;
use std::convert::Infallible;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type DB = mongodb::Database;

mod app;
mod db;
mod error;
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

    let _collection = db.collection("books");

    // TODO: put routing in a module
    let health_route = warp::path!("health")
        .and(with_db(db.clone()))
        .and_then(web::handler::health_handler);
    let metrics_route = warp::path!("metrics").and_then(web::handler::metrics_handler);
    let welcome_route = warp::path::end()
        .and(with_db(db.clone()))
        .and_then(app::welcome_handler);

    let routes = welcome_route
        .or(metrics_route)
        .or(health_route)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    info!("Started on port 8080");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
