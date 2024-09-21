use actix_web::{web, App, HttpServer};
use fm_lib::util::mongo::mongo;
use fm_server::approve_invitation_code_route::approve_invitation_code_route;
use fm_server::create_approved_user_route::create_unapproved_user_route;
use fm_server::create_buyer_route::create_buyer_route;
use fm_server::create_seller_route::create_seller_route;
use fm_server::State;
use std::sync::{Arc, RwLock};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();
    let state = web::Data::new(State{
        mongo: mongo().await,
        buyers: Arc::new(RwLock::new(vec![])),
        sellers: Arc::new(RwLock::new(vec![])),
    });

    HttpServer::new(move|| {
        App::new()
            .app_data(state.clone())
            .service(create_seller_route)
            .service(create_buyer_route)
            .service(approve_invitation_code_route)
            .service(create_unapproved_user_route)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}








