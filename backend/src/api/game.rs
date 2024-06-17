use actix_web::{get, web::Json};

use crate::model::grid::{Cell, Grid};

#[get("/game/{game_id}")]
pub async fn get_game() -> Json<String> {
    let mut grid = Grid::new(2);
    grid.play([1, 5].to_vec(), Cell::Cross);
    grid.play([5, 7].to_vec(), Cell::Circle);
    grid.play([7, 3].to_vec(), Cell::Cross);
    Json(grid.export())
}
