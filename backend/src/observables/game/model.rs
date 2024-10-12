use crate::{db::DatabaseError, errors::ApplicationError, observables::grid::model::Grid, utils::generate_id};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub grid: Grid,
    pub current_player: i16
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
struct GameDTO {
    id: String,
    grid: String,
    current_player: i16
}

impl Game {
    // Creates a game, saves it in the db and returns it
    pub async fn new(pool: &Pool<Postgres>) -> Result<Self, ApplicationError> {
        const DEPTH: u8 = 2;

        let new_game = Game {
            id: generate_id(),
            grid: Grid::new(DEPTH)?,
            current_player: 0
        };

        let result = sqlx::query_as::<_, GameDTO>(
            r#"
            INSERT INTO games (id, grid, current_player)
            VALUES ($1, $2, $3)
            RETURNING *;
            "#,
            )
            .bind(new_game.id.clone())
            .bind(new_game.grid.export())
            .bind(new_game.current_player)
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
            SELECT id, grid, current_player FROM games
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
                    grid: grid?,
                    current_player: game_dto.current_player
                })
            },
            Err(_) => Err(ApplicationError::Database(DatabaseError::NotFound("Grid".to_string())))
        }
    }

    // Saves the current game in the db
    pub async fn save(self: &Self, pool: &Pool<Postgres>) -> Result<(), ApplicationError> {
        let result = sqlx::query(
            r#"
            UPDATE games SET
            grid = $2,
            current_player = $3
            WHERE id = $1
            RETURNING *;
            "#,
            )
            .bind(self.id.clone())
            .bind(self.grid.export())
            .bind(self.current_player)
            .fetch_one(pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ApplicationError::Database(DatabaseError::Update("Grid".to_string())))
        }
    }
}
