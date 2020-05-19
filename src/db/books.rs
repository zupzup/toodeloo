use crate::app::books::NewBook;
use crate::data::Book;
use crate::{error::Error::*, Result, DB};
use bson::doc;
use bson::ordered::OrderedDocument;
use chrono::prelude::*;
use futures::StreamExt;

const BOOKS: &str = "books";
const ID: &str = "_id";
const NAME: &str = "name";
const AUTHOR: &str = "author";
const LANG: &str = "language";
const NUM_PAGES: &str = "num_pages";
const ADDED_AT: &str = "added_at";

pub async fn fetch_books(db: &DB) -> Result<Vec<Book>> {
    let coll = db.collection(BOOKS);

    let mut cursor = coll.find(None, None).await.map_err(MongoQueryError)?;
    let mut result: Vec<Book> = Vec::new();

    while let Some(doc) = cursor.next().await {
        result.push(doc_to_book(&doc?)?);
    }
    Ok(result)
}

pub async fn create_book(entry: &NewBook, db: &DB) -> Result<()> {
    let coll = db.collection(BOOKS);
    let doc = doc! {
        NAME: entry.name.clone(),
        AUTHOR: entry.author.clone(),
        LANG: entry.language.clone(),
        NUM_PAGES: entry.pages,
        ADDED_AT: Utc::now(),
    };
    coll.insert_one(doc, None).await.map_err(MongoQueryError)?;
    Ok(())
}

fn doc_to_book(doc: &OrderedDocument) -> Result<Book> {
    let id = doc.get_object_id(ID)?;
    let name = doc.get_str(NAME)?;
    let author = doc.get_str(AUTHOR)?;
    let lang = doc.get_str(LANG)?;
    println!("{:?}", doc.get(NUM_PAGES));
    let num_pages = doc.get_i32(NUM_PAGES)?;
    let added_at = doc.get_utc_datetime(ADDED_AT)?;

    let book = Book::new(
        &id.to_hex(),
        name,
        author,
        lang,
        num_pages as usize,
        added_at,
    );
    Ok(book)
}
