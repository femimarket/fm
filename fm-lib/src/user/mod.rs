use mongodb::Database;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use crate::util::constants::USERS;
use crate::util::mongo::mongo;
use crate::util::secret::sign_message;
use crate::util::user::get_invited_user;

#[derive(Clone,Serialize,Deserialize)]
pub struct User {
    pub id: String,
    pub invitation_code: String,
    pub approved: bool,
}

pub async fn approve_invitation_code(
    mongo:&Database,
    invitation_code:&str
)->User{
    let mut user = get_invited_user(
        mongo,
        invitation_code
    ).await.unwrap();
    user.approved = true;
    mongo.collection::<User>(USERS).insert_many(
        vec![
            &user
        ]
    ).await.unwrap();
    user
}

pub async fn create_unapproved_user(
    mongo:&Database,
) -> User {
    let user_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let token = sign_message(
        user_id.as_ref()
    );
    let token_str = hex::encode(token);
    let invitation_code = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let user = User {
        id: token_str,
        invitation_code,
        approved: false,
    };
    let mongo = mongo().await;
    mongo.collection::<User>("users").insert_many(
        vec![
            &user
        ]
    ).await.unwrap();
    user
}