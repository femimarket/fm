
use hmac::digest::MacError;
use mongodb::bson::{doc, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use rand::distributions::{Alphanumeric, DistString};

use crate::util::constants::SELLERS;
use crate::util::secret::{sign_message, verify_signed_message};
use crate::util::user::{get_approved_user, verify_user};
use crate::util::verify_and_insert_many;

#[derive(Clone,Serialize,Deserialize)]
pub struct Seller {
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub price: i64,
    pub from_lat: f64,
    pub from_lng: f64,
    pub from_rad: f64,
    pub from_time_start: i64,
    pub from_time_end: i64,
}


pub async fn create_seller(
    mongo:&Database,
    user: &Seller
) -> bool {
    verify_and_insert_many::<Seller>(
        mongo,
        &user.user_id,
        SELLERS,
        user.clone()
    ).await
}