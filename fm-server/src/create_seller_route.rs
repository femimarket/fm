use std::ops::Deref;
use actix_web::{get, post, web, Responder};
use fm_lib::seller::{create_seller, Seller};
use crate::State;

#[get("/test2")]
async fn create_seller_route(
    state: web::Data<State>,
    user:web::Json<Seller>,
) -> actix_web::Result<impl Responder> {

    Ok("".to_string())
}