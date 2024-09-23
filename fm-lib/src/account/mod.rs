use crate::util::near::near_secret_file;
use crate::util::switch_crypto_account;
use crate::State;
use near_workspaces::types::NearToken;
use near_workspaces::{AccountId, InMemorySigner};
use std::str::FromStr;
use std::sync::Arc;

pub mod oanda;
pub mod near;


pub async fn transfer_crypto(
    state:&Arc<State>,
    to:&str,
    amount:i64,
    crypto_account: &str
){
    match switch_crypto_account(&crypto_account).as_str() {
        "near" => {
            let n = state.near_client.read().unwrap();
            let signer = InMemorySigner::from_file(near_secret_file().as_ref()).unwrap();
            let _ = n.transfer_near(
                &signer,
                &to.parse().unwrap(),
                NearToken::from_near(amount as u128)
            ).await.unwrap().unwrap();
        },
        _ => unreachable!()
    }
}

pub async fn view_crypto_account(
    state:&Arc<State>,
    account:&str,
    crypto_account: &str
)->i64{
    match switch_crypto_account(&crypto_account).as_str() {
        "near" => {
            let n = state.near_client.read().unwrap();
            let near_account = AccountId::from_str(account).unwrap();
            let b = n.view_account(
                &near_account
            ).await.unwrap();
            b.balance.as_near() as i64
        },
        _ => unreachable!()
    }
}