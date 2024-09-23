use std::sync::{Arc, RwLock};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use mongodb::Database;
use rand::distributions::{Alphanumeric, DistString};
use crate::account::near::{create_near_account, generate_near_account};
use crate::account::oanda::{create_oanda_account, OandaAccount};
use crate::State;
use crate::user::{User, UserPayment};
use crate::util::constants::{OANDA_ACCOUNTS, OANDA_USERS, USERS};
use crate::util::mongo::{create, get, update_by_id};
use crate::util::secret::sign_message;

#[derive(Clone,Serialize,Deserialize)]
pub struct OandaUser {
    pub id: String,
    pub invitation_code: String,
    pub approved: bool,
    pub payment: UserPayment,
}



pub async fn approve_oanda_user_mongo(
    mongo:&Database,
    invitation_code:&str,
    oanda_account: &OandaAccount
)->OandaUser{
    let mut user = get::<OandaUser>(
        mongo,
        OANDA_USERS,
        doc! {"invitation_code":invitation_code}
    ).await.unwrap();

    assert_eq!(user.id, oanda_account.user_id);

    user.approved = true;

    create::<OandaAccount>(mongo,OANDA_ACCOUNTS,oanda_account).await;

    update_by_id::<OandaUser>(
        mongo,
        OANDA_ACCOUNTS,
        &user.id,
        &user
    ).await;

    user
}



