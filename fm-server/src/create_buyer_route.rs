use std::ops::Deref;
use actix_web::{post, web, Responder};
use fm_lib::buyer::{create_buyer, Buyer};
use crate::State;

#[post("/create-buyer")]
async fn create_buyer_route(
    state: web::Data<State>,
    user:web::Json<Buyer>,
) -> actix_web::Result<impl Responder> {
   
    Ok("".to_string())
}