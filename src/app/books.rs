use crate::db::books::{create_book, delete_book, edit_book, fetch_book, fetch_books};
use crate::{
    data::{Book, Session},
    error::Error::*,
    WebResult, DB,
};
use askama::Template;
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

#[derive(Template)]
#[template(path = "book/edit.html")]
struct EditBookTemplate {
    book: Book,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBook {
    pub name: String,
    pub author: String,
    pub language: String,
    pub pages: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditedBook {
    pub name: String,
    pub author: String,
    pub language: String,
    pub pages: i32,
}

pub async fn books_list_handler(session: Session, db: DB) -> WebResult<impl Reply> {
    log::info!("session in handler: {:?}", session);
    let books = fetch_books(&db).await.map_err(|e| reject::custom(e))?;
    let template = BooklistTemplate { books };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn new_book_handler(_session: Session, _db: DB) -> WebResult<impl Reply> {
    let template = NewBookTemplate {};
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn create_book_handler(session: Session, body: NewBook, db: DB) -> WebResult<impl Reply> {
    create_book(&body, &db)
        .await
        .map_err(|e| reject::custom(e))?;
    books_list_handler(session, db).await
}

pub async fn edit_book_handler(_session: Session, id: String, db: DB) -> WebResult<impl Reply> {
    let book = fetch_book(&id, &db).await.map_err(|e| reject::custom(e))?;
    let template = EditBookTemplate { book };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

pub async fn do_edit_book_handler(
    session: Session,
    id: String,
    body: EditedBook,
    db: DB,
) -> WebResult<impl Reply> {
    edit_book(&id, &body, &db)
        .await
        .map_err(|e| reject::custom(e))?;
    books_list_handler(session, db).await
}

pub async fn delete_book_handler(session: Session, id: String, db: DB) -> WebResult<impl Reply> {
    delete_book(&id, &db).await.map_err(|e| reject::custom(e))?;
    books_list_handler(session, db).await
}
