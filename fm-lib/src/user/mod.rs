use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use crate::util::mongo::mongo;
use crate::util::secret::sign_message;

#[derive(Clone,Serialize,Deserialize)]
pub struct User {
    pub id: String,
    pub approved: bool,
}

pub async fn create_user() -> User {
    let user_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let token = sign_message(
        user_id.as_ref()
    );

    let token_str = hex::encode(token);

    let user = User {
        id: token_str,
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