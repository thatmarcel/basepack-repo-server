use std::env;

use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

static mut CLIENT: Option<Client> = None;

pub async fn initialize_database() {
    unsafe {
        let client_options = ClientOptions::parse(
            env::var("MONGO_URL").expect("Missing MongoDB connection string")
        ).await.unwrap();
        
        CLIENT = Some(Client::with_options(client_options).unwrap());
    }
}

pub async fn get_database() -> Database {
    unsafe {
        let client = CLIENT.as_ref().expect("No database client instance exists");
        client.database("bp-v1")
    }
}