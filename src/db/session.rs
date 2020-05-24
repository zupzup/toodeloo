use crate::data::Session;
use crate::{error::Error::*, Result, DB};
use bson::ordered::OrderedDocument;
use bson::{doc, oid::ObjectId};
use uuid::Uuid;

const SESSIONS: &str = "sessions";
const ID: &str = "_id";
const SESSION_ID: &str = "session_id";
const USER_ID: &str = "user_id";

pub async fn create_session(user_id: &str, db: &DB) -> Result<String> {
    let coll = db.collection(SESSIONS);
    let oid = ObjectId::with_string(user_id).map_err(|_| InvalidIDError(user_id.to_owned()))?;
    let session_id = Uuid::new_v4();
    let doc = doc! {
        SESSION_ID: session_id.to_string(),
        USER_ID: oid,
    };
    coll.insert_one(doc, None).await.map_err(MongoQueryError)?;
    Ok(session_id.to_string())
}

pub async fn find_session(session_id: &str, db: &DB) -> Result<Session> {
    let coll = db.collection(SESSIONS);
    let filter = doc! {
        SESSION_ID: session_id,
    };

    let result = coll.find_one(filter, None).await.map_err(MongoQueryError)?;
    match result {
        Some(v) => {
            let session = doc_to_session(&v)?;
            Ok(session)
        }
        None => Err(NoEntryFoundError(session_id.to_owned())),
    }
}

pub async fn delete_session(session_id: &str, db: &DB) -> Result<()> {
    let coll = db.collection(SESSIONS);
    let filter = doc! {
        SESSION_ID: session_id,
    };
    coll.delete_one(filter, None)
        .await
        .map_err(MongoQueryError)?;
    Ok(())
}

fn doc_to_session(doc: &OrderedDocument) -> Result<Session> {
    let id = doc.get_object_id(ID)?;
    let session_id = doc.get_str(SESSION_ID)?;
    let user_id = doc.get_object_id(USER_ID)?;

    let session = Session {
        id: id.to_hex(),
        session_id: session_id.to_owned(),
        user_id: user_id.to_hex(),
    };
    Ok(session)
}
