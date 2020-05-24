use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{
    http::{StatusCode, Uri},
    redirect, reply, Rejection, Reply,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::ordered::ValueAccessError),
    #[error("could find entry for: {0}")]
    NoEntryFoundError(String),
    #[error("invalid id used: {0}")]
    InvalidIDError(String),
    #[error("templating error: {0}")]
    TemplateError(#[from] askama::Error),
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
    #[error("invalid credentials used")]
    InvalidCredentials,
    #[error("could not create session")]
    CreateSessionError,
    #[error("could not log out")]
    LogoutError,
    #[error("no session found")]
    NoSessionFoundError,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
    let code;
    let message;
    // TODO: create template for this
    // on invalid body, should return to the previous page with an error, depending

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::NoSessionFoundError => {
                return Ok(Box::new(redirect(Uri::from_static("/login"))));
            }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(Box::new(reply::with_status(json, code)))
}
