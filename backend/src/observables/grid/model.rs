use std::{collections::VecDeque, vec};

use crate::errors::ApplicationError;

use super::errors::{CellError, GridError};

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Circle,
    Cross,
    Both,
    SubGrid(Grid)
}

impl Cell {
    pub fn from_bits(bits: &str, optional_subgrid: Option<Grid>) -> Result<Self, ApplicationError> {
        match bits {
           "000" => Ok(Cell::Empty),
           "001" => Ok(Cell::Circle),
           "010" => Ok(Cell::Cross),
           "011" => Ok(Cell::Both),
           "100" => {
               if let Some(subgrid) = optional_subgrid {
                   Ok(Cell::SubGrid(subgrid))
                } else {
                    Err(ApplicationError::Grid(GridError::NoSubgrid()))
                }
           },
            _ => Err(ApplicationError::Cell(CellError::Load()))
        }
    }

    pub fn to_bits(self: &Self) -> &str {
        match self {
            Self::Empty => "000",
            Self::Circle => "001",
            Self::Cross => "010",
            Self::Both => "011",
            Self::SubGrid(_) => "100"
        }
    }

    pub fn get_subgrid(&self) -> Option<&Grid> {
        if let Cell::SubGrid(grid) = self {
            Some(grid)
        } else {
            None
        }
    }

    pub fn get_mut_subgrid(self: &mut Self) -> Option<&mut Grid> {
        if let Cell::SubGrid(grid) = self {
            Some(grid)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
     pub cells: Vec<Cell>,
}

pub type Position = Vec<usize>;

impl Grid {
    // create an empty grid
    pub fn new(depth: u8) -> Result<Self, ApplicationError> {
        if depth < 1 {
            return Err(ApplicationError::Grid(GridError::InvalidDepth(depth)));
        }
        if depth == 1 {
            Ok(Grid {
                cells: vec![Cell::Empty; 9]
            })
        } else {
            Ok(Grid {
                cells: vec![Cell::SubGrid(Grid::new(depth - 1)?); 9]
            })
        }
    }

    // create grid from bytes string
    pub fn load(bytes_string: String) -> Result<Grid, ApplicationError> {
        if bytes_string.len() % 27 != 0 {
            return Err(ApplicationError::Grid(GridError::InvalidLength(bytes_string.len())));
        }

        let mut queue:VecDeque<Grid> = VecDeque::new();

        let subgrid_number = bytes_string.len() / 27;
        // Processing the subgrid in the reverse order
        for subgrid_index in 0..subgrid_number {
            let i = subgrid_number - subgrid_index;
            let substr = &bytes_string[(i-1)*27..(i)*27];

            let mut tmp_grid: Grid = Grid { cells: vec![Cell::Empty; 9] };

            // Loading cell from end to start
            for j in (0..27).step_by(3) {
                let k = 27 - j;
                let bits = &substr[k-3..k];

                let sg: Option<Grid> = if bits == "100" {
                    queue.pop_front()
                } else { None };

                let cell = Cell::from_bits(bits, sg)?;
                tmp_grid.cells[(k/3)-1] = cell;
            }
            queue.push_back(tmp_grid);
        }

        if queue.len() != 1 {
            return Err(ApplicationError::Grid(GridError::InvalidBytesString()));
        }

        match queue.pop_front() {
            Some(main_grid) => Ok(main_grid),
            None => Err(ApplicationError::Grid(GridError::InvalidBytesString()))
        }
    }

    // export bytes string
    pub fn export(self: &Self) -> String {
        let mut bytes_string = String::from("");

        let mut grids_queue: VecDeque<&Grid> = VecDeque::from([self]);

        while let Some(current_grid) = grids_queue.pop_front() {
            for cell in &current_grid.cells {
                bytes_string += cell.to_bits();
                if let Some(subgrid) = cell.get_subgrid() {
                    grids_queue.push_back(subgrid);
                }
            }
        }

        bytes_string
    }

    // play a move
    pub fn play(self: &mut Self, position: &Position, cell_value: Cell) -> Result<(), ApplicationError> {
        let depth = position.len();
        if depth == 0 {
            return Err(ApplicationError::Grid(GridError::InvalidPosition("Empty position".to_string())));
        }

        let mut current_grid = self;

        // Looping through every subgrid
        for &index in position.iter().take(depth - 1) {
            if let Some(cell) = current_grid.cells.get_mut(index) {
                if let Some(subgrid) = cell.get_mut_subgrid() {
                    current_grid = subgrid;
                } else {
                    return Err(ApplicationError::Grid(GridError::InvalidPosition("Playing too deep".to_string())));
                }
            } else {
                return Err(ApplicationError::Grid(GridError::InvalidPosition("Out of grid cell".to_string())));
            }
        }

        let played_index = *position.last().unwrap();
        if let Some(targeted_cell) = current_grid.cells.get_mut(played_index) {
            if *targeted_cell != Cell::Empty {
                return Err(ApplicationError::Cell(CellError::AlreadyUsed()));
            }
            *targeted_cell = cell_value;
        } else {
            return Err(ApplicationError::Grid(GridError::InvalidPosition("Out of grid cell".to_string())));
        }

        Ok(())
    }

    // sanitize the grid
    // => convert a finished subgrid to its new value (cross, circle, both)

    // check result
}
