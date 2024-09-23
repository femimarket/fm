use std::sync::{Arc, RwLock};
use mongodb::Database;
use near_workspaces::{InMemorySigner, Worker};
use near_workspaces::network::{Custom, Mainnet};
use near_workspaces::types::NearToken;
use crate::account::near::NearAccount;
use crate::account::oanda::OandaAccount;
use crate::user::oanda::OandaUser;
use crate::user::User;
use crate::util::near::{near_network, near_secret_file};

pub mod util;
pub mod user;
pub mod buyer;
pub mod courier;
pub mod seller;
pub mod instrument;
pub mod tick;
pub mod trade;
pub mod account;
pub mod cli;

#[derive(Debug)]
pub struct State {
    pub near_client:Arc<RwLock<Worker<Custom>>>,
    pub users:Arc<RwLock<Vec<User>>>,
    pub oanda_accounts:Arc<RwLock<Vec<OandaAccount>>>,
    pub near_accounts:Arc<RwLock<Vec<NearAccount>>>,
}

pub fn state()->State {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let near_client = runtime.block_on(async {
        let w = near_workspaces::custom(
            match near_network().as_str() {
                "localnet" => "http://localhost:3030",
                rest => panic!("unknown network {}",rest)
            }
        ).await.unwrap();
        let signer = InMemorySigner::from_file(near_secret_file().as_ref()).unwrap();
        let near_account = serde_json::from_str::<NearAccount>(
            &std::fs::read_to_string(near_secret_file()).unwrap()
        ).unwrap();
        let _ = w.transfer_near(
            &signer,
            &near_account.account_id.parse().unwrap(),
            NearToken::from_near(100)
        ).await.unwrap();
        w
    });

    State{
        near_client: Arc::new(RwLock::new(near_client)),
        users: Arc::new(RwLock::new(vec![])),
        oanda_accounts: Arc::new(RwLock::new(vec![])),
        near_accounts: Arc::new(RwLock::new(vec![])),
    }
}
