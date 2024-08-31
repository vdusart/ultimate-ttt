use actix_web::{
    get, post,
    web::{Json, Path, Data}
};

use crate::{model::game::Game, ApplicationState};

#[post("/game")]
pub async fn create_game(data: Data<ApplicationState>) -> Json<String> {
    let game = match Game::new(&data.pool).await {
        Ok(game) => game,
        Err(error_msg) => {
            println!("{}", error_msg);
            todo!("Return an http error with the error_msg in it.");
        }
    };
    Json(game.id)
}

#[get("/game/{game_id}")]
pub async fn get_game(data: Data<ApplicationState>, path: Path<String>) -> Json<String> {
    let game_id = path.into_inner();
    println!("The requested id is: {}", game_id);
    let game = match Game::load(&data.pool, game_id).await {
        Ok(game) => game,
        Err(_error_msg) => todo!("Return an http error with the error_msg in it."),
    };
    Json(game.grid.export())
}
