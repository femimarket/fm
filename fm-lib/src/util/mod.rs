mod server;
pub mod mongo;
pub mod secret;
pub mod constants;
pub mod user;

use std::collections::HashMap;
use hmac::digest::MacError;
use hmac::{Hmac, Mac};
use mongodb::Database;
use serde::Serialize;
use serde_json::Value;
use sha2::Sha256;
use crate::util::mongo::mongo;
use crate::util::user::{get_approved_user, verify_user};

pub fn fm_encode(data: impl Serialize) -> String {
    let mut result:HashMap<&String,Value> = HashMap::new();
    let json = serde_json::to_string(&data).unwrap();
    let h = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let mut keys = h.keys().into_iter().collect::<Vec<_>>();
    keys.sort();
    for x in keys {
        result.entry(x).or_insert(h[x].clone());
    }
    serde_json::to_string(&result).unwrap()
}

pub async  fn verify_and_insert_many<T:Send+Sync+Serialize>(
    mongo:&Database,
    user_id:&str,
    collection_name:&str,
    data:T
)->bool{
    let res = get_approved_user(
        mongo,
        user_id
    ).await;
    match (res,verify_user(user_id)) {
        (Some(_), Ok(_)) => {
            mongo.collection::<T>(collection_name).insert_many(vec![
                data
            ]).await.unwrap();
            true
        }
        _ => false
    }
}
