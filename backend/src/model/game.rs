use crate::{db, schema::games, utils::generate_id};
use diesel::prelude::*;

use super::grid::Grid;

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub grid: Grid,
}

#[derive(Debug, Queryable)]
struct GameDTO {
    id: String,
    grid: String,
}

impl Game {
    // Creates a game, saves it in the db and returns it
    pub fn new(depth: u8) -> Result<Self, String> {
        use crate::schema::games::dsl::*;
        let error_msg: String = String::from("Impossible to create a game.");

        let new_game = Game {
            id: generate_id(),
            grid: Grid::new(depth)
        };

        let mut connection = db::init();
        let insert_result = diesel::insert_into(games)
            .values((id.eq(&new_game.id), grid.eq(new_game.grid.export())))
            .execute(&mut connection);

        match insert_result {
            Ok(inserted_rows) => {
                if inserted_rows != 1 {
                    Err(error_msg)
                } else {
                    Ok(new_game)
                }
            }
            Err(_) => Err(String::from(error_msg))
        }
    }

    // Loads a game from a game_id
    pub fn load(game_id: String) -> Result<Self, String> {
        use crate::schema::games::dsl::*;
        let mut connection = db::init();

        match games.filter(id.eq(game_id)).first::<GameDTO>(&mut connection) {
            Ok(game_dto) => Ok(Game {
                id: game_dto.id,
                // TODO: load the proper grid from the grid string
                grid: Grid::new(1)
            }),
            Err(_) => Err(String::from("Impossible to load game."))
        }
    }

    // save -> Updates the game in the db
}
