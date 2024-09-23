use std::str::FromStr;
use std::sync::Arc;
use clap::builder::TypedValueParser;
use near_workspaces::network::{Custom, Sandbox};
use near_workspaces::types::{NearToken, SecretKey};
use near_workspaces::{AccountId, InMemorySigner, Worker};
use tokio::sync::OnceCell;

const NEAR_NETWORK: std::cell::OnceCell<String> = std::cell::OnceCell::new();
const NEAR_SIGNER: std::cell::OnceCell<InMemorySigner> = std::cell::OnceCell::new();

const NEAR_SECRET: std::cell::OnceCell<String> = std::cell::OnceCell::new();
const NEAR_SECRET_FILE: std::cell::OnceCell<String> = std::cell::OnceCell::new();
const NEAR_ACCOUNT_ID: std::cell::OnceCell<String> = std::cell::OnceCell::new();
pub const NEAR_LOCALNET: OnceCell<Arc<Worker<Custom>>> = OnceCell::const_new();

pub fn near_secret() -> String {
    NEAR_SECRET.get_or_init(|| {
        std::env::var("NEAR_SECRET").unwrap()
    }).to_string()
}

pub fn near_secret_file() -> String {
    NEAR_SECRET_FILE.get_or_init(|| {
        std::env::var("NEAR_SECRET_FILE").unwrap()
    }).to_string()
}

pub fn near_account_id() -> String {
    NEAR_ACCOUNT_ID.get_or_init(|| {
        std::env::var("NEAR_ACCOUNT_ID").unwrap()
    }).to_string()
}

pub fn near_network() -> String {
    NEAR_NETWORK.get_or_init(|| {
        std::env::var("NEAR_NETWORK").unwrap()
    }).to_string()
}

pub async fn near_localnet() -> Arc<Worker<Custom>> {
    NEAR_LOCALNET.get_or_init( || async {
        let a = near_workspaces::mainnet().await.unwrap();
        let w = near_workspaces::custom("http://localhost:3030").await.unwrap();
        let signer = InMemorySigner::from_file(near_secret_file().as_ref()).unwrap();
        let _ = w.transfer_near(
            &signer,
            &near_account_id().parse().unwrap(),
            NearToken::from_near(100)
        ).await.unwrap();
        println!("good vibes");
        Arc::new(w)
    }).await.clone()
}


pub async fn near_balance(
    near_client:Worker<Custom>,
    account_str:&str
)->u128{
    println!("a a a {:?}",account_str);
    let near_account = AccountId::from_str(account_str).unwrap();
    match near_network().as_str() {
        "localnet" => {
            let r = near_client.view_account(&near_account).await.unwrap();
            r.balance.as_near()
        },
        _ => panic!("Network doesn't exist")
    }
}

// pub async fn near_transfer(
//     account_str:&str,
//     amount:i64,
// ){
//     let near_account = AccountId::from_str(account_str).unwrap();
//     match near_network().as_str() {
//         "localnet" => {
//             let worker = near_localnet().await;
//             let root = worker.root_account().unwrap();
//             let d = root.transfer_near(
//                 &near_account,
//                 NearToken::from_near((100) as u128)
//             ).await.unwrap().unwrap();
//             let aa = worker.view_account(&near_account).await.unwrap();
//             println!("{:?}", aa);
//         },
//         _ => panic!("Network doesn't exist")
//     }
// }
