use super::model::Game;
use crate::{observables::grid::model::Cell, ApplicationState};
use actix_web::{
    http::StatusCode,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;

pub struct GameController {}

#[derive(Debug, Deserialize)]
pub struct PlayRequest {
    position: Vec<usize>,
}

impl GameController {
    pub async fn create_game(data: Data<ApplicationState>) -> impl Responder {
        match Game::new(&data.pool).await {
            Ok(game) => HttpResponse::build(StatusCode::CREATED).json(game.id),
            Err(error) => error.error_response(),
        }
    }

    pub async fn get_game(data: Data<ApplicationState>, path: Path<String>) -> impl Responder {
        let game_id = path.into_inner();
        match Game::load(&data.pool, game_id).await {
            Ok(game) => HttpResponse::build(StatusCode::OK).json(game.grid.export()),
            Err(error) => error.error_response(),
        }
    }

    pub async fn play(
        data: Data<ApplicationState>,
        path: Path<String>,
        play_request: Json<PlayRequest>,
    ) -> impl Responder {
        let game_id = path.into_inner();
        let result = Game::load(&data.pool, game_id).await;

        if let Err(error) = result {
            return error.error_response();
        }

        let mut game = result.unwrap();

        let cell_value = if game.current_player == 0 {
            Cell::Circle
        } else {
            Cell::Cross
        };
        let result = game.grid.play(&play_request.position, cell_value);
        if let Err(error) = result {
            return error.error_response();
        }

        for i in 1..play_request.position.len() {
            game.grid
                .sanitize(&play_request.position[0..play_request.position.len() - i]);
        }

        game.current_player = (game.current_player + 1) % 2;

        let result = game.save(&data.pool).await;
        if let Err(error) = result {
            return error.error_response();
        }

        HttpResponse::build(StatusCode::OK).json("move played")
    }
}
