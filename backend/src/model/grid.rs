#[derive(Clone, Debug)]
pub enum Cell {
    Empty,
    Circle,
    Cross,
    Both,
    SubGrid(Grid)
}

#[derive(Clone, Debug)]
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
    // play a move
    // check result
}
