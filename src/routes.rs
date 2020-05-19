use crate::{app, error, web, DB};
use std::convert::Infallible;
use warp::Filter;

pub fn router(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    let health_route = warp::path!("health")
        .and(with_db(db.clone()))
        .and_then(web::handler::health_handler);
    let metrics_route = warp::path!("metrics").and_then(web::handler::metrics_handler);

    let welcome_route = warp::path::end()
        .and(with_db(db.clone()))
        .and_then(app::welcome_handler);

    let books = warp::path("books");
    let new = warp::path("new");
    let edit = warp::path("edit");
    let delete = warp::path("delete");

    let books_routes = books
        .and(new)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(app::books::new_book_handler)
        .or(books
            .and(new)
            .and(warp::post())
            .and(warp::body::form())
            .and(with_db(db.clone()))
            .and_then(app::books::create_book_handler))
        .or(books
            .and(edit)
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(app::books::edit_book_handler))
        .or(books
            .and(delete)
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(app::books::delete_book_handler))
        .or(books
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(app::books::books_list_handler));

    welcome_route
        .or(metrics_route)
        .or(health_route)
        .or(books_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection)
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
