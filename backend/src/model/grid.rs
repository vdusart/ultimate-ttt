use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Circle,
    Cross,
    Both,
    SubGrid(Grid)
}

impl Cell {
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
     pub cells: Vec<Cell>,
}

impl Grid {
    // create an empty grid
    pub fn new(depth: u8) -> Self {
        if depth == 1 {
            Grid {
                cells: vec![Cell::Empty; 9]
            }
        } else {
            Grid {
                cells: vec![Cell::SubGrid(Grid::new(depth - 1)); 9]
            }
        }
    }

    // create grid from bytes string
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
    // check result
}
