use crate::db::{session::create_session, user::fetch_user};
use crate::{error::Error::*, WebResult, DB};
use askama::Template;
use serde::{Deserialize, Serialize};
use warp::{reject, reply::html, Reply};

const SET_COOKIE: &str = "Set-Cookie";
const COOKIE_NAME: &str = "toodeloo";
const MAX_AGE: usize = 60 * 60 * 24 * 15; // 15 days

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[derive(Template)]
#[template(path = "loggedin.html")]
struct LoggedInTemplate {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub async fn login_handler() -> WebResult<impl Reply> {
    let template = LoginTemplate {};
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn do_login_handler(body: LoginUser, db: DB) -> WebResult<impl Reply> {
    let user = fetch_user(&body.email, &db)
        .await
        .map_err(|_| reject::custom(InvalidCredentials))?;

    match bcrypt::verify(&body.password, &user.password) {
        Ok(v) => {
            if !v {
                return Err(reject::custom(InvalidCredentials));
            }
        }
        Err(_) => return Err(reject::custom(InvalidCredentials)),
    };
    let session_id = create_session(&user.id, &db)
        .await
        .map_err(|_| reject::custom(CreateSessionError))?;
    let template = LoggedInTemplate { email: body.email };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    let html = warp::reply::html(res);
    // TODO: encrypt session_id in cookie
    let response = warp::reply::with_header(html, SET_COOKIE, &create_cookie(&session_id));
    Ok(response)
}

fn create_cookie(session_id: &str) -> String {
    let cookie = format!(
        "{}={};Max-Age={};HTTPOnly;Secure",
        COOKIE_NAME, session_id, MAX_AGE
    );
    cookie
}
