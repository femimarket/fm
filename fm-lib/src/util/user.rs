use hmac::digest::MacError;
use mongodb::bson::doc;
use mongodb::Database;
use crate::user::User;
use crate::util::constants::{SELLERS, USERS};
use crate::util::secret::{secret, verify_signed_message};

pub async fn get_approved_user(
    mongo:&Database,
    id: &str
) -> Option<User> {
    let res = mongo.collection::<User>(USERS).find_one(
        doc! {"id": id, "approved":true},
    ).await.unwrap();
    res
}

pub async fn get_invited_user(
    mongo:&Database,
    invitation_code: &str
) -> Option<User> {
    let res = mongo.collection::<User>(USERS).find_one(
        doc! {"invitation_code":invitation_code},
    ).await.unwrap();
    res
}

pub fn verify_user(user_id: &str) -> Result<(), MacError> {
    verify_signed_message(&hex::decode(user_id).unwrap())
}

pub fn is_admin(secret: &str) -> bool {
    secret == secret()
}