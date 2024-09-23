use std::sync::{Arc, RwLock};
use actix_web::{web, App, HttpResponse, test};
use fm_server::{server_config};
use actix_web::http::header::ContentType;
use actix_web::http::Method;
use fm_lib::account::oanda::OandaAccount;
use fm_lib::state;
use fm_lib::util::mongo::mongo;
use fm_server::create_unapproved_user_route::create_unapproved_user_request;
use fm_server::approve_user_route::approve_user_request;
use fm_server::transfer_crypto_route::transfer_crypto_request;
use fm_server::view_crypto_account_route::view_crypto_account_request;

#[actix_web::test]
async fn test_index_get() {

}

#[actix_web::test]
async fn sign_up() {
    let host = "http://localhost:8081";
    let trade_account = "oanda";

    let user = create_unapproved_user_request(
        host,
        trade_account,
    ).await;
    println!("{:#?}", user);

    transfer_crypto_request(
        host,
        &user.near_address,
        user.near_amount,
        "near"
    ).await;

    let balance = view_crypto_account_request(
        host,
        &user.near_address,
        "near"
    ).await;

    println!("{:#?}", balance);


    let oanda_account =  OandaAccount{
        account_id: "".to_string(),
        secret: "".to_string(),
        id: "".to_string(),
        user_id: user.id.clone(),
        live:false
    };

    let oanda_account = serde_json::to_value(&oanda_account).unwrap();
    let s = approve_user_request(
        host,
        &user.invitation_code,
        trade_account,
        &oanda_account
    ).await;

    println!("{:?}", s);

}