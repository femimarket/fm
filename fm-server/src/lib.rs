use std::sync::{Arc, RwLock};
use mongodb::Database;
use fm_lib::buyer::Buyer;
use fm_lib::seller::Seller;

pub mod create_seller_route;
pub mod create_buyer_route;
pub mod approve_invitation_code_route;
pub mod create_approved_user_route;

pub struct State {
    pub mongo:Database,
    pub buyers:Arc<RwLock<Vec<Buyer>>>,
    pub sellers:Arc<RwLock<Vec<Seller>>>,
}