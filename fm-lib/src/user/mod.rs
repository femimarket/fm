pub mod oanda;

use std::sync::Arc;
use mongodb::Database;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::account::near::{create_near_account, generate_near_account};
use crate::account::oanda::OandaAccount;
use crate::State;
use crate::user::oanda::OandaUser;
use crate::util::constants::OANDA_ACCOUNTS;
use crate::util::mongo::create;
use crate::util::secret::sign_message;
use crate::util::{switch_crypto_account, switch_trade_account};
use crate::util::near::near_balance;

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct User {
    pub id: String,
    pub invitation_code: String,
    pub approved: bool,
    pub trade_account: String,
    pub near_amount:i64,
    pub eth_amount:i64,
    pub near_address:String,
    pub eth_address:String,
}

#[derive(Clone,Serialize,Deserialize)]
pub struct UserPayment {
    pub near_amount:f64,
    pub eth_amount:f64,
    pub near_address:String,
    pub eth_address:String,
}


pub async fn create_unapproved_user(
    state: &Arc<State>,
    trade_account:&str,
) -> User {

    let _ = switch_trade_account(trade_account);
    let mut users = state.users.write().unwrap();

    let random_str = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let client_secret = sign_message(
        random_str.as_ref()
    );
    let client_secret_str = hex::encode(client_secret);
    let invitation_code = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let near_account  = generate_near_account().await;
    {
        let mut near_accounts = state.near_accounts.write().unwrap();
        near_accounts.push(near_account.clone());
    }

    let user = User {
        id: client_secret_str,
        invitation_code,
        approved: false,
        trade_account: trade_account.to_string(),
        near_amount: 5000,
        near_address: near_account.account_id,
        eth_amount:1,
        eth_address:"".to_string(),
    };

    {
        users.push(user.clone());
    }

    user
}

pub async fn approve_user(
    state:&Arc<State>,
    invitation_code:&str,
    account: &Value
)->User{

    let mut users = state.users.write().unwrap();
    let mut user = users.iter_mut().find(|u| u.invitation_code == invitation_code).expect("No such user: invitation_code");

    match switch_trade_account(&user.trade_account).as_str() {
        "oanda" => {
            let oanda_account  = serde_json::from_value::<OandaAccount>(account.clone()).unwrap();
            assert_eq!(user.id, oanda_account.user_id);
            user.approved = true;

            let near_client = state.near_client.read().unwrap();
            let near_balance_u128 = near_balance(
                near_client.clone(),
                &user.near_address
            ).await;

            if (near_balance_u128 as i64) < user.near_amount {
                panic!("balance:{near_balance_u128} < {}",user.near_amount)
            }


            {
                let mut oanda_accounts = state.oanda_accounts.write().unwrap();
                oanda_accounts.push(oanda_account.clone());
            }

            user.clone()
        },
        _ => unreachable!()
    }
}



