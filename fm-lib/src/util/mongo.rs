use std::cell::OnceCell;
use mongodb::bson::{doc, Document};
use mongodb::{bson, Client, Database};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion, UpdateModifications};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

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
    let client = Client::with_uri_str(&mongo_secret()).await.unwrap();
    // Ping the server to see if you can connect to the cluster
    // client
    //     .database("admin")
    //     .run_command(doc! {"ping": 1})
    //     .await.unwrap();
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    client.database("fm")
}

pub async fn create<T:Send+Sync+Serialize>(
    mongo:&Database,
    collection_name:&str,
    data:&T,
){
    mongo.collection::<T>(collection_name).insert_many(
        vec![
            data
        ]
    ).await.unwrap();
}

pub async fn get<T:Send+Sync+Serialize+DeserializeOwned>(
    mongo:&Database,
    collection_name:&str,
    doc: Document
) -> Option<T> {
    let res = mongo.collection::<T>(collection_name).find_one(
        doc,
    ).await.unwrap();
    res
}

pub async fn update_by_id<T:Send+Sync+Serialize>(
    mongo:&Database,
    collection_name:&str,
    id:&str,
    data:&T
) {
    let d  = bson::to_document(&data).unwrap();
    mongo.collection::<T>(collection_name).update_one(
        doc! { "id": id },
        d
    ).await.unwrap();
}