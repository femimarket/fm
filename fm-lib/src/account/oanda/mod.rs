use mongodb::Database;
use serde::{Deserialize, Serialize};
use crate::util::constants::OANDA_ACCOUNTS;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OandaAccount {
    pub account_id: String,
    pub secret: String,
    pub id: String,
    pub user_id: String,
    pub live: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OandaLiveAccount {
    pub account_id: String,
    pub secret: String,
    pub test_account_id: String,
}


pub async fn create_oanda_account(
    mongo:&Database,
    oanda_account: &OandaAccount
){
    mongo.collection::<OandaAccount>(OANDA_ACCOUNTS).insert_many(
        vec![
            oanda_account
        ]
    ).await.unwrap();
}