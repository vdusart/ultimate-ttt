use actix_web::{http::StatusCode, web::{Data, Path}, HttpResponse, Responder};
use crate::ApplicationState;
use super::model::Game;

pub struct GameController {}

impl GameController {
    pub async fn create_game(data: Data<ApplicationState>) -> impl Responder {
        match Game::new(&data.pool).await {
            Ok(game) => HttpResponse::build(StatusCode::CREATED).json(game.id),
            Err(error) => error.error_response()
        }
    }

    pub async fn get_game(data: Data<ApplicationState>, path: Path<String>) -> impl Responder {
        let game_id = path.into_inner();
        match Game::load(&data.pool, game_id).await {
            Ok(game) => HttpResponse::build(StatusCode::OK).json(game.grid.export()),
            Err(error) => error.error_response()
        }
    }
}
