use actix_web::{get, post, web::{Json, Path}};

use crate::model::{game::Game, grid::{Cell, Grid}};

#[post("/game")]
pub async fn create_game() -> Json<String> {
    let game = match Game::new(1) {
        Ok(game) => game,
        Err(error_msg) => todo!("Return an http error with the error_msg in it.")
    };
    Json(game.id)
}

#[get("/game/{game_id}")]
pub async fn get_game(path: Path<String>) -> Json<String> {
    let game_id = path.into_inner();
    println!("The requested id is: {}", game_id);
    let game = match Game::load(game_id) {
        Ok(game) => game,
        Err(error_msg) => todo!("Return an http error with the error_msg in it.")
    };
    Json(game.grid.export())
}