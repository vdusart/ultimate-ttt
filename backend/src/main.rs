mod api;
mod db;
mod model;
mod schema;
mod utils;

use std::{env, io};

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

use api::game;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3001");

        App::new()
            .wrap(cors)
            .wrap(Logger::new("[%t] - %r (%s) - %Dms"))
            .service(hello)
            .service(game::get_game)
            .service(game::create_game)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
