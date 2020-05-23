use crate::data::User;
use crate::{error::Error::*, Result, DB};
use bson::doc;
use bson::ordered::OrderedDocument;

const USERS: &str = "users";
const ID: &str = "_id";
const EMAIL: &str = "email";
const PASSWORD: &str = "password";

pub async fn fetch_user(email: &str, db: &DB) -> Result<User> {
    let coll = db.collection(USERS);

    let filter = doc! {
        EMAIL: email,
    };

    let result = coll.find_one(filter, None).await.map_err(MongoQueryError)?;
    match result {
        Some(v) => {
            let book = doc_to_user(&v)?;
            Ok(book)
        }
        None => Err(NoEntryFoundError(email.to_owned())),
    }
}

fn doc_to_user(doc: &OrderedDocument) -> Result<User> {
    let id = doc.get_object_id(ID)?;
    let email = doc.get_str(EMAIL)?;
    let password = doc.get_str(PASSWORD)?;

    let user = User {
        id: id.to_hex(),
        email: email.to_owned(),
        password: password.to_owned(),
    };
    Ok(user)
}
