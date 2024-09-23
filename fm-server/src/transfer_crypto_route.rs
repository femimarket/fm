use actix_web::{post, web, Responder};
use fm_lib::account::transfer_crypto;
use serde::Deserialize;
use fm_lib::{state, State};

#[derive(Deserialize)]
struct  Params {
    to: String,
    amount: i64,
    crypto_account: String,
}
#[post("/transfer-crypto")]
pub async fn transfer_crypto_route(
    state: web::Data<State>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    transfer_crypto(
        &state,
        &params.to,
        params.amount,
        &params.crypto_account,
    ).await;
    Ok(web::Json(()))
}


pub async fn transfer_crypto_request(
    host:&str,
    to: &str,
    amount: i64,
    crypto_account: &str,
){
    let client = reqwest::Client::new();
    let url = format!("{host}/transfer-crypto?crypto_account={crypto_account}&amount={amount}&to={to}",);
    client.post(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}