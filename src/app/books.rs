use crate::{data::Book, error::Error::*, WebResult, DB};
use askama::Template;
use warp::{reject, reply::html, Reply};

#[derive(Template)]
#[template(path = "book/list.html")]
struct BooklistTemplate {
    books: Vec<Book>,
}

pub async fn books_list_handler(_db: DB) -> WebResult<impl Reply> {
    let books = vec![
        Book::new("1", "Siddharta", "Hermann Hesse", "DE", 200),
        Book::new("2", "Sei Du Selbst", "Richard David Precht", "DE", 550),
    ];
    let template = BooklistTemplate { books };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}
