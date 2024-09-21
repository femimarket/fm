use crate::State;
use actix_web::{post, web, Responder};
use fm_lib::user::{create_unapproved_user, User};
use fm_lib::util::user::is_admin;

#[post("/create-unapproved-user")]
async fn create_unapproved_user_route(
    state: web::Data<State>,
    user:web::Json<User>,
) -> actix_web::Result<impl Responder> {
    match is_admin(&user.id) {
        true => {
            let user = create_unapproved_user(
                &state.mongo
            ).await;
            Ok(user.invitation_code.to_string())
        }
        false => Ok("".to_string())
    }
}