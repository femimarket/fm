use std::cell::OnceCell;
use mongodb::bson::doc;
use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use serde::Serialize;

const MONGO_SECRET: OnceCell<String> = OnceCell::new();

fn mongo_secret() -> String {
    MONGO_SECRET.get_or_init(|| {
        std::env::var("MONGO_SECRET").unwrap()
    }).to_string()
}

pub async fn mongo() -> Database {
    let mut client_options =
        ClientOptions::parse(&mongo_secret()).await.unwrap();
    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Get a handle to the cluster
    let client = Client::with_options(client_options).unwrap();
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await.unwrap();
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    client.database("fm")
}

