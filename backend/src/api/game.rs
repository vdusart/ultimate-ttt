use actix_web::{get, web::Json};

use crate::model::grid::Grid;

#[get("/game/{game_id}")]
pub async fn get_game() -> Json<String> {
    let grid = Grid::new(2);
    Json(grid.export())
}
