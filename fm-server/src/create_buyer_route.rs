use std::ops::Deref;
use actix_web::{post, web, Responder};
use fm_lib::seller::{create_seller, Seller};
use crate::State;

#[post("/create-buyer")]
async fn create_buyer_route(
    state: web::Data<State>,
    user:web::Json<Buyer>,
) -> actix_web::Result<impl Responder> {
    match create_buyer(
        &state.mongo,
        user.deref(),
    ).await {
        true => {
            {
                state.buyers.write().unwrap().push(user.into_inner());
            }
        }
        false => {}
    }
    Ok("".to_string())
}