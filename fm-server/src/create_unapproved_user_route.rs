use std::ops::Deref;
use crate::State;
use actix_web::{post, web, Responder};
use serde::Deserialize;
use fm_lib::user::{create_unapproved_user, User};
use fm_lib::util::user::is_admin;


#[derive(Deserialize)]
struct  Params {
    trade_account: String,
}
#[post("/create-unapproved-user")]
pub async fn create_unapproved_user_route(
    state: web::Data<State>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    let user = create_unapproved_user(
        &state,
        params.trade_account.as_str(),
    ).await;
    Ok(web::Json(user))
}


pub async fn create_unapproved_user_request(
    host:&str,
    trade_account: &str,
) -> User {
    let client = reqwest::Client::new();
    let url = format!("{host}/create-unapproved-user?trade_account={trade_account}",);
    client.post(&url)
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap()
}