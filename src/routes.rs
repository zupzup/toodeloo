use crate::data::Session;
use crate::db::session::find_session;
use crate::{app, error, web, WebResult, DB};
use std::convert::Infallible;
use warp::{reject, Filter, Rejection};

const COOKIE_NAME: &str = "toodeloo";

pub fn router(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    let health_route = warp::path!("health")
        .and(with_db(db.clone()))
        .and_then(web::handler::health_handler);
    let metrics_route = warp::path!("metrics").and_then(web::handler::metrics_handler);

    let welcome_route = warp::path::end().and_then(app::welcome_handler);

    let books = warp::path("books");
    let new = warp::path("new");
    let list = warp::path("list");
    let edit = warp::path("edit");
    let delete = warp::path("delete");
    let login = warp::path("login");

    let auth_routes = login
        .and(warp::get())
        .and_then(app::auth::login_handler)
        .or(login
            .and(warp::post())
            .and(warp::body::form())
            .and(with_db(db.clone()))
            .and_then(app::auth::do_login_handler));

    let books_routes = books
        .and(new)
        .and(warp::get())
        .and(with_valid_session(db.clone()))
        .and(with_db(db.clone()))
        .and_then(app::books::new_book_handler)
        .or(books
            .and(new)
            .and(warp::post())
            .and(warp::body::form())
            .and(with_valid_session(db.clone()))
            .and(with_db(db.clone()))
            .and_then(app::books::create_book_handler))
        .or(books
            .and(edit)
            .and(warp::get())
            .and(warp::path::param())
            .and(with_valid_session(db.clone()))
            .and(with_db(db.clone()))
            .and_then(app::books::edit_book_handler))
        .or(books
            .and(edit)
            .and(warp::post())
            .and(warp::path::param())
            .and(warp::body::form())
            .and(with_valid_session(db.clone()))
            .and(with_db(db.clone()))
            .and_then(app::books::do_edit_book_handler))
        .or(books
            .and(delete)
            .and(warp::get())
            .and(warp::path::param())
            .and(with_valid_session(db.clone()))
            .and(with_db(db.clone()))
            .and_then(app::books::delete_book_handler))
        .or(books
            .and(list)
            .and(warp::get())
            .and(with_valid_session(db.clone()))
            .and(with_db(db.clone()))
            .and_then(app::books::books_list_handler));

    welcome_route
        .or(auth_routes)
        .or(metrics_route)
        .or(health_route)
        .or(books_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection)
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn do_stuff(inp: (String, DB)) -> WebResult<Session> {
    let cookie = inp.0;
    let db = inp.1;
    // TODO: if session invalid, move to login page
    log::info!("cookie: {}", cookie);
    let session = find_session(&cookie, &db)
        .await
        .map_err(|_| reject::custom(error::Error::NoSessionFoundError))?;
    Ok(session)
}

fn with_valid_session(db: DB) -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    warp::cookie(COOKIE_NAME)
        .map(move |cookie: String| (cookie.clone(), db.clone()))
        .and_then(do_stuff)
}
