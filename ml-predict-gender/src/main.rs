mod models;
mod trainer;

use std::{env};
use mongodb::{
    options::{ClientOptions},
    sync::{Client}
};
use anyhow::{Result};

use crate::{models::{DbNameRecord}, trainer::train_neural_network};

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

fn predict_gender_loop() -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    let client = connect_to_mongo()?;
    let db = client.database("ml-data");
    let names = db.collection::<DbNameRecord>("names");

    //TODO: store neural network on disk, and check if neural network exists before retraining it
    train_neural_network(&names)?;

    predict_gender_loop()?;

    Ok(())
}
