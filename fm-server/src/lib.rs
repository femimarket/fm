use std::sync::{Arc, RwLock};
use actix_web::{web, HttpResponse};
use actix_web::web::{service, Data};
use mongodb::Database;
use fm_lib::account::near::NearAccount;
use fm_lib::buyer::Buyer;
use fm_lib::seller::Seller;
use fm_lib::State;
use fm_lib::user::oanda::OandaUser;
use fm_lib::util::mongo::mongo;

pub mod create_seller_route;
pub mod create_buyer_route;
pub mod approve_user_route;
pub mod create_unapproved_user_route;
pub mod transfer_crypto_route;
pub mod view_crypto_account_route;

pub fn server_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_seller_route::create_seller_route)
        .service(create_buyer_route::create_buyer_route)
        .service(approve_user_route::approve_user_route)
        .service(create_unapproved_user_route::create_unapproved_user_route)
        .service(view_crypto_account_route::view_crypto_account_route)
        .service(transfer_crypto_route::transfer_crypto_route);

}

