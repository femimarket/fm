use std::ops::Deref;
use actix_web::{post, web, Responder};
use fm_lib::seller::{create_seller, Seller};

#[post("/create-seller")]
async fn create_seller_route(
    state: web::Data<State>,
    user:web::Json<Seller>,
) -> actix_web::Result<impl Responder> {
    match create_seller(
        &state.mongo,
        user.deref(),
    ).await {
        true => {
            {
                state.sellers.write().unwrap().push(user.into_inner());
            }
        }
        false => {}
    }
    Ok("".to_string())
}