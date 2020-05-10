use crate::{error::Error::*, WebResult, DB};
use warp::{reject, reply::html, Reply};
use yarte::Template;

#[derive(Template)]
#[template(path = "welcome")]
struct WelcomeTemplate<'a> {
    title: &'a str,
    body: &'a str,
}

pub mod books;

pub async fn welcome_handler(_db: DB) -> WebResult<impl Reply> {
    let template = WelcomeTemplate {
        title: "Welcome",
        body: "To Toodeloo!",
    };
    let res = template
        .call()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}
