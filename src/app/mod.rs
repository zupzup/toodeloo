use crate::data::Session;
use crate::{error::Error::*, WebResult};
use askama::Template;
use warp::{reject, reply::html, Reply};

#[derive(Template)]
#[template(path = "welcome.html")]
struct WelcomeTemplate<'a> {
    title: &'a str,
    body: &'a str,
}

pub mod auth;
pub mod books;

pub async fn welcome_handler(_session: Session) -> WebResult<impl Reply> {
    let template = WelcomeTemplate {
        title: "Welcome",
        body: "To Toodeloo!",
    };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}
