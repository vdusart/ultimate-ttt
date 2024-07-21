use crate::{db, utils::generate_id};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::grid::Grid;

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub grid: Grid,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
struct GameDTO {
    id: String,
    grid: String,
}

impl Game {
    // Creates a game, saves it in the db and returns it
    pub async fn new(depth: u8) -> Result<Self, String> {
        let error_msg: String = String::from("Impossible to create a game.");

        let new_game = Game {
            id: generate_id(),
            grid: Grid::new(depth)
        };

        let pool = db::get_pool().await;

        let result = sqlx::query_as::<_, GameDTO>(
            r#"
            INSERT INTO games (id, grid)
            VALUES ($1, $2)
            RETURNING *;
            "#,
            )
            .bind(new_game.id.clone())
            .bind(new_game.grid.export())
            .fetch_one(&pool)
            .await;

        match result {
            Ok(_) => Ok(new_game),
            Err(_) => Err(error_msg)
        }
    }

    // Loads a game from a game_id
    pub async fn load(game_id: String) -> Result<Self, String> {
        let pool = db::get_pool().await;

        let result = sqlx::query_as::<_, GameDTO>(
            r#"
            SELECT * FROM games
            WHERE id=$1;
            "#,
            )
            .bind(game_id)
            .fetch_one(&pool)
            .await;

        match result {
            Ok(game_dto) => {
                // todo: Here we have to load a grid from its bytes string representation
                let grid = Grid::new(1);
                Ok(Game {
                    id: game_dto.id,
                    grid: grid
                })
            },
            Err(_) => Err(String::from("Impossible to load game."))
        }
    }

    // Saves the current game in the db
    pub async fn save(self: &Self) -> Result<(), String> {
        let error_msg: String = String::from("Impossible to create a game.");

        let pool = db::get_pool().await;
        
        let result = sqlx::query(
            r#"
            UPDATE games SET
            grid = $2 WHERE id = $1
            RETURNING *;
            "#,
            )
            .bind(self.id.clone())
            .bind(self.grid.export())
            .fetch_one(&pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(error_msg)
        }
    }
}
