use actix_web::{web, App, HttpServer};
use fm_lib::util::mongo::mongo;
use fm_server::approve_user_route::approve_user_route;
use fm_server::create_unapproved_user_route::create_unapproved_user_route;
use fm_server::create_buyer_route::create_buyer_route;
use fm_server::create_seller_route::create_seller_route;
use fm_server::{server_config};
use std::sync::{Arc, RwLock};
use std::thread;
use actix_web::web::Data;
use fm_lib::state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();
    let state = web::Data::new(
        thread::spawn(|| {
            state()
        }).join().expect("Thread panicked")
    );
    HttpServer::new(move|| {
        App::new()
            .app_data(state.clone())
            .configure(server_config)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}













