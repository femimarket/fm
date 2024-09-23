use std::ops::Deref;
use crate::State;
use actix_web::{post, web, Responder};
use serde::Deserialize;
use serde_json::json;
use fm_lib::account::view_crypto_account;
use fm_lib::user::{create_unapproved_user, User};
use fm_lib::util::user::is_admin;


#[derive(Deserialize)]
struct  Params {
    address: String,
    crypto_account: String,
}
#[post("/view-crypto-account")]
pub async fn view_crypto_account_route(
    state: web::Data<State>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    let balance = view_crypto_account(
        &state,
        &params.address,
        &params.crypto_account,
    ).await;
    Ok(balance.to_string())
}


pub async fn view_crypto_account_request(
    host:&str,
    address: &str,
    crypto_account: &str,
)->i64{
    let client = reqwest::Client::new();
    let url = format!("{host}/view-crypto-account?crypto_account={crypto_account}&address={address}");
    client.post(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .parse()
        .unwrap()
}