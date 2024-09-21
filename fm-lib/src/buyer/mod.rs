
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use rand::distributions::{Alphanumeric, DistString};
use crate::user::User;
use crate::util::constants::{BUYERS, SELLERS, USERS};
use crate::util::secret::{sign_message, verify_signed_message};
use crate::util::user::{get_approved_user, verify_user};
use crate::util::verify_and_insert_many;

#[derive(Clone,Serialize,Deserialize)]
pub struct Buyer {
    pub from_lat: f64,
    pub from_lng: f64,
    pub from_rad: i64,
    pub to_lat: f64,
    pub to_lng: f64,
    pub to_rad: i64,
    pub to_time_start: i64,
    pub to_time_end: i64,
    pub price_start: i64,
    pub price_end: i64,
    pub user_id: String
}


pub async fn create_buyer(
    mongo:&Database,
    user: Buyer
)  {
    verify_and_insert_many::<Buyer>(
        mongo,
        &user.user_id,
        BUYERS,
        user.clone()
    ).await;
}