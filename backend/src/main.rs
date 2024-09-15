mod db;
mod errors;
mod observables;
mod utils;

use std::io;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use dotenv::dotenv;
use observables::game::router::GameRouter;
use sqlx::{Pool, Postgres};

struct ApplicationState {
    pool: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::get_pool(&database_url).await;

    let _ = sqlx::migrate!("./migrations")
        .run(&pool)
        .await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3001");

        App::new()
            .wrap(cors)
            .wrap(Logger::new("[%t] - %r (%s) - %Dms"))
            .app_data(web::Data::new(ApplicationState { pool: pool.clone() }))
            .configure(GameRouter::register_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
