use std::ops::Deref;
use actix_web::{post, web, Responder};
use fm_lib::seller::{create_seller, Seller};
use fm_lib::user::{approve_invitation_code, User};
use crate::State;

#[post("/approve-invitation-code")]
async fn approve_invitation_code_route(
    state: web::Data<State>,
    user:web::Json<User>,
) -> actix_web::Result<impl Responder> {
    let user = approve_invitation_code(
        &state.mongo,
        &user.invitation_code
    ).await;
    Ok(user.id.to_string())
}