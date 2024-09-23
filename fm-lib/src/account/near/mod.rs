use std::str::FromStr;
use mongodb::bson::doc;
use mongodb::Database;
use near_workspaces::InMemorySigner;
use near_workspaces::types::{KeyType, SecretKey};
use serde::{Deserialize, Serialize};
use crate::util::constants::{NEAR_ACCOUNTS, USERS};
use crate::util::near::{near_account_id, near_localnet, near_network, near_secret};

#[derive(Serialize,Debug,Deserialize,Clone)]
pub struct NearAccount {
    pub secret_key:String,
    pub public_key:String,
    pub account_id:String,
}

pub async fn generate_near_account() -> NearAccount {
    let sk = SecretKey::from_random(KeyType::ED25519);
    let pk = sk.public_key();
    let pk_str=pk.to_string().replace("ed25519:", "");
    let pk_b58 = bs58::decode(pk_str).into_vec().unwrap();
    let pk_hex = hex::encode(pk_b58);
    match near_network().as_str() {
        "localnet" => {
            NearAccount {
                secret_key:sk.to_string(),
                public_key:pk.to_string(),
                account_id:pk_hex,
            }
        }
        _ => panic!("Near account not found")
    }
}

pub async fn create_near_account(
    mongo:&Database,
    near_account: &NearAccount
){
    let res = mongo.collection::<NearAccount>(NEAR_ACCOUNTS).insert_many(
        vec![
            near_account
        ],
    ).await.unwrap();
}

pub async fn get_near_account(
    mongo:&Database,
    pk: &str
) -> Option<NearAccount> {
    let res = mongo.collection::<NearAccount>(NEAR_ACCOUNTS).find_one(
        doc! {"pk":pk},
    ).await.unwrap();
    res
}