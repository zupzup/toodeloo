use crate::{Result, CONFIG};
use log::info;
use mongodb::{options::ClientOptions, Client, Database};

pub mod books;
pub mod session;
pub mod user;

pub async fn init() -> Result<(Database, Client)> {
    let mut client_options =
        ClientOptions::parse(&format!("mongodb://{}:{}", CONFIG.db.host, CONFIG.db.port)).await?;

    client_options.app_name = Some("Toodeloo".to_string());
    info!(
        "Connecting to MongoDB at {}:{}",
        CONFIG.db.host, CONFIG.db.port
    );
    let client = Client::with_options(client_options)?;
    let db = client.database(&CONFIG.db.name);

    if CONFIG.app.init_db {
        info!("Initializing collections...");
        // initialize books
        // initialize users
        // initialize sessions
    }

    Ok((db, client))
}
