use std::str;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::info;

use url_shortener::hasher::StringHash;
use url_shortener::kvs::KVS;

#[get("/{key}")]
async fn redirect(path: web::Path<String>, kvs: web::Data<KVS>) -> impl Responder {
    // TBD
}

#[get("/shorten/{url:.*}")]
async fn shorten(path: web::Path<String>, state: web::Data<KVS>) -> impl Responder {
    // TDB
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // BONUS: load KVS from file.

    let kvs_data = web::Data::new(
    // TBD: create the KVS
    );

    let web = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(kvs_data)
            .service(shorten)
            .service(redirect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    // BNUS: dump KVS to a file.
    Ok(())
}
