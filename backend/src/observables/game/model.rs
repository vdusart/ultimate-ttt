use crate::{db::DatabaseError, errors::ApplicationError, observables::grid::model::Grid, utils::generate_id};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

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
    pub async fn new(pool: &Pool<Postgres>) -> Result<Self, ApplicationError> {
        const DEPTH: u8 = 2;

        let new_game = Game {
            id: generate_id(),
            grid: Grid::new(DEPTH)?
        };

        let result = sqlx::query_as::<_, GameDTO>(
            r#"
            INSERT INTO games (id, grid)
            VALUES ($1, $2)
            RETURNING *;
            "#,
            )
            .bind(new_game.id.clone())
            .bind(new_game.grid.export())
            .fetch_one(pool)
            .await;


        match result {
            Ok(_) => Ok(new_game),
            Err(_) => Err(ApplicationError::Database(DatabaseError::Insert("Game".to_string())))
        }
    }

    // Loads a game from a game_id
    pub async fn load(pool: &Pool<Postgres>, game_id: String) -> Result<Self, ApplicationError> {
        let result = sqlx::query_as::<_, GameDTO>(
            r#"
            SELECT id, grid FROM games
            WHERE id=$1;
            "#,
            )
            .bind(game_id)
            .fetch_one(pool)
            .await;

        match result {
            Ok(game_dto) => {
                let grid = Grid::load(game_dto.grid);
                Ok(Game {
                    id: game_dto.id,
                    grid: grid?
                })
            },
            Err(_) => Err(ApplicationError::Database(DatabaseError::NotFound("Grid".to_string())))
        }
    }

    // Saves the current game in the db
    pub async fn save(self: &Self, pool: &Pool<Postgres>) -> Result<(), String> {
        let error_msg: String = String::from("Impossible to create a game.");
        
        let result = sqlx::query(
            r#"
            UPDATE games SET
            grid = $2 WHERE id = $1
            RETURNING *;
            "#,
            )
            .bind(self.id.clone())
            .bind(self.grid.export())
            .fetch_one(pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(error_msg)
        }
    }
}
