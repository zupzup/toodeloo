use crate::db::books::{create_book, fetch_books};
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
    pub name: String,
    pub author: String,
    pub language: String,
    pub pages: i32,
}

pub async fn books_list_handler(db: DB) -> WebResult<impl Reply> {
    info!("in list books handler");
    let books = fetch_books(&db).await.map_err(|e| reject::custom(e))?;
    let template = BooklistTemplate { books };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn new_book_handler(_db: DB) -> WebResult<impl Reply> {
    info!("in new book handler");
    let template = NewBookTemplate {};
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn create_book_handler(body: NewBook, db: DB) -> WebResult<impl Reply> {
    info!("in create book handler");
    create_book(&body, &db)
        .await
        .map_err(|e| reject::custom(e))?;
    books_list_handler(db).await
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
