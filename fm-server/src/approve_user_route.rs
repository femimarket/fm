use std::ops::Deref;
use actix_web::{post, web, Responder};
use serde::Deserialize;
use serde_json::Value;
use fm_lib::account::oanda::OandaAccount;
use fm_lib::user::{approve_user, User};
use fm_lib::user::oanda::{ OandaUser};
use crate::State;

#[derive(Deserialize)]
struct Params {
    code: String,
    trade_account:String,
}

#[post("/approve-user")]
async fn approve_user_route(
    state: web::Data<State>,
    params: web::Query<Params>,
    account:web::Json<serde_json::Value>,
) -> actix_web::Result<impl Responder> {
    let user = approve_user(
        &state,
        &params.code,
        account.deref()
    ).await;
    Ok(web::Json(user))
}

pub async fn approve_user_request(
    host:&str,
    code:&str,
    trade_account: &str,
    account:&Value,
) -> String {
    let client = reqwest::Client::new();
    let url = format!("{host}/approve-user?code={code}&trade_account={trade_account}",);
    client.post(&url)
        .json(&account)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}