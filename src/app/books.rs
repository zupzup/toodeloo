use crate::{data::Book, error::Error::*, WebResult, DB};
use askama::Template;
use log::info;
use serde::{Deserialize, Serialize};
use warp::{reject, reply::html, Reply};

#[derive(Template)]
#[template(path = "book/list.html")]
struct BooklistTemplate {
    books: Vec<Book>,
}

#[derive(Template)]
#[template(path = "book/new.html")]
struct NewBookTemplate {}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBook {
    name: String,
    author: String,
    language: String,
    pages: usize,
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

pub async fn new_book_handler(_db: DB) -> WebResult<impl Reply> {
    let template = NewBookTemplate {};
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn create_book_handler(body: NewBook, _db: DB) -> WebResult<impl Reply> {
    info!("inside create book handler {:?}", body);
    let template = BooklistTemplate { books: vec![] };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn edit_book_handler(id: i32, db: DB) -> WebResult<impl Reply> {
    let template = NewBookTemplate {};
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn delete_book_handler(id: i32, db: DB) -> WebResult<impl Reply> {
    let template = NewBookTemplate {};
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}
