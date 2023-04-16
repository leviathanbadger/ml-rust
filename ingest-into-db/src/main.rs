mod name_data;

use std::{env};
use anyhow::{Result};
use mongodb::{
    options::{ClientOptions},
    sync::{Client}
};
use name_data::{ingest_name_data};

fn connect_to_mongo() -> Result<Client> {
    let username = env::var("MONGODB_USERNAME")?;
    let password = env::var("MONGODB_PASSWORD")?;
    let host = env::var("MONGODB_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("MONGODB_PORT").unwrap_or("27017".to_string());
    let uri = format!("mongodb://{}:{}@{}:{}", username, password, host, port);
    let client_options = ClientOptions::parse(uri)?;

    let client = Client::with_options(client_options)?;

    Ok(client)
}

fn main() -> Result<()> {
    let client = connect_to_mongo()?;
    let db = client.database("ml-data");

    ingest_name_data(db)?;

    Ok(())
}
