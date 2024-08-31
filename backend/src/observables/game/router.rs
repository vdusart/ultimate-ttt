use actix_web::web;

use super::controller::GameController;

pub struct GameRouter {}

impl GameRouter {
   pub fn register_routes(cfg: &mut web::ServiceConfig) {
       cfg.service(
           web::resource("/game")
           .route(web::post().to(GameController::create_game))
        );

       cfg.service(
           web::resource("/game/{game_id}")
           .route(web::get().to(GameController::get_game))
        );
   }
}
