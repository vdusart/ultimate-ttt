#[cfg(test)]
mod tests {
    use crate::{errors::ApplicationError, observables::grid::{errors::{CellError, GridError}, model::{Cell, Grid}}};

    // Testing the "new" function
    #[test]
    fn test_new_invalid_depth() {
        const INVALID_DEPTH: u8 = 0;
        let result = Grid::new(INVALID_DEPTH);
        assert!(result.is_err());

        if let Err(ApplicationError::Grid(GridError::InvalidDepth(depth))) = result {
            assert_eq!(depth, INVALID_DEPTH);
        } else {
            panic!("Expected GridError::InvalidDepth, but got a different error");
        }
    }

    #[test]
    fn test_new_depth_one() {
        let result = Grid::new(1);
        assert!(result.is_ok());

        let grid = result.unwrap();
        assert_eq!(grid.cells.len(), 9);
        assert!(grid.cells.iter().all(|cell| matches!(cell, Cell::Empty)));
    }

    #[test]
    fn test_new_depth_two() {
        let result = Grid::new(2);
        assert!(result.is_ok());

        let grid = result.unwrap();
        assert_eq!(grid.cells.len(), 9);

        // Each subgrid contains 9 "Cell::Empty" cells
        for cell in grid.cells {
            match cell {
                Cell::SubGrid(sub_grid) => {
                    assert_eq!(sub_grid.cells.len(), 9);
                    assert!(sub_grid.cells.iter().all(|cell| matches!(cell, Cell::Empty)));
                },
                _ => panic!("Expected Cell::SubGrid, but found another type"),
            }
        }
    }


    // Testing the "load" function
    #[test]
    fn test_load_empty_grid() {
        let bytes_string = "000000000000000000000000000".to_string();

        let result = Grid::load(bytes_string);
        assert!(result.is_ok());

        let loaded_grid = result.unwrap();
        assert_eq!(loaded_grid.cells.len(), 9);

        let expected_grid = Grid {
            cells: vec![Cell::Empty; 9]
        };

        assert_eq!(loaded_grid, expected_grid);
    }

    #[test]
    fn test_load_with_subgrid() {
        let subgrid1_1 = Grid {
            cells: vec![
                Cell::Circle, Cell::Cross, Cell::Empty,
                Cell::Empty, Cell::Circle, Cell::Circle,
                Cell::Empty, Cell::Cross, Cell::Empty,
            ]
        };

        let subgrid1 = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::SubGrid(subgrid1_1),
            ]
        };

        let subgrid2 = Grid {
            cells: vec![
                Cell::Empty, Cell::Cross, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Circle,
                Cell::Empty, Cell::Cross, Cell::Empty,
            ]
        };

        let subgrid3 = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Circle, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let subgrid4 = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Circle,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let expected_grid = Grid {
            cells: vec![
                Cell::Circle, Cell::SubGrid(subgrid1), Cell::SubGrid(subgrid2),
                Cell::Circle, Cell::SubGrid(subgrid3), Cell::Cross,
                Cell::Cross, Cell::SubGrid(subgrid4), Cell::Circle,
            ]
        };

        let bytes_string = "001100100001100010010100001000000000000000000000000100000010000000000001000010000000000000001000000000000000000000001000000000000000000001010000000001001000010000".to_string();

        let result = Grid::load(bytes_string);
        assert!(result.is_ok());

        let loaded_grid = result.unwrap();

        assert_eq!(loaded_grid, expected_grid);
    }


    // Testing the "export" function
    #[test]
    fn test_export_empty_grid() {
       let grid = Grid {
           cells: vec![Cell::Empty; 9]
       };

       let bytes_string = grid.export();
       assert_eq!(bytes_string, "000000000000000000000000000");
    }

    #[test]
    fn test_export_with_subgrids() {
        let subgrid1_1 = Grid {
            cells: vec![
                Cell::Circle, Cell::Cross, Cell::Empty,
                Cell::Empty, Cell::Circle, Cell::Circle,
                Cell::Empty, Cell::Cross, Cell::Empty,
            ]
        };
        assert_eq!(subgrid1_1.export(), "001010000000001001000010000");


        let subgrid1 = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::SubGrid(subgrid1_1),
            ]
        };
        assert_eq!(subgrid1.export(), "000000000000000000000000100001010000000001001000010000");


        let subgrid2 = Grid {
            cells: vec![
                Cell::Empty, Cell::Cross, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Circle,
                Cell::Empty, Cell::Cross, Cell::Empty,
            ]
        };
        assert_eq!(subgrid2.export(), "000010000000000001000010000");


        let subgrid3 = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Circle, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };
        assert_eq!(subgrid3.export(), "000000000001000000000000000");


        let subgrid4 = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Circle,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };
        assert_eq!(subgrid4.export(), "000000001000000000000000000");


        let grid = Grid {
            cells: vec![
                Cell::Circle, Cell::SubGrid(subgrid1), Cell::SubGrid(subgrid2),
                Cell::Circle, Cell::SubGrid(subgrid3), Cell::Cross,
                Cell::Cross, Cell::SubGrid(subgrid4), Cell::Circle,
            ]
        };

        let bytes_string = grid.export();
        assert_eq!(bytes_string, "001100100001100010010100001000000000000000000000000100000010000000000001000010000000000000001000000000000000000000001000000000000000000001010000000001001000010000");
    }

    // Testing the full "load/export" system
    #[test]
    fn test_full_load_export_system() {
        let bytes_string = "001100100001100010010100001000000000000000000000000100000010000000000001000010000000000000001000000000000000000000001000000000000000000001010000000001001000010000";

        let result = Grid::load(bytes_string.to_string());
        assert!(result.is_ok());

        let loaded_grid = result.unwrap();

        let export_string = loaded_grid.export();
        assert_eq!(bytes_string, export_string);
    }

    #[test]
    fn test_play_empty_position() {
        let mut grid = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let position: Vec<usize> = vec![];
        let result = grid.play(&position, Cell::Cross);
        assert!(result.is_err());

        if let Err(ApplicationError::Grid(GridError::InvalidPosition(_))) = result {
            assert!(true);
        } else {
            panic!("Expected GridError::InvalidPosition, but got a different error");
        }
    }

    #[test]
    fn test_play_invalid_position() {
        let mut grid = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let position: Vec<usize> = vec![10];
        let result = grid.play(&position, Cell::Cross);
        assert!(result.is_err());

        if let Err(ApplicationError::Grid(GridError::InvalidPosition(_))) = result {
            assert!(true);
        } else {
            panic!("Expected GridError::InvalidPosition, but got a different error");
        }
    }
    
    #[test]
    fn test_play_invalid_deep_position() {
        let subgrid = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };
        let mut grid = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::SubGrid(subgrid),
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let position: Vec<usize> = vec![2, 10];
        let result = grid.play(&position, Cell::Cross);
        assert!(result.is_err());

        if let Err(ApplicationError::Grid(GridError::InvalidPosition(_))) = result {
            assert!(true);
        } else {
            panic!("Expected GridError::InvalidPosition, but got a different error");
        }
    }

    #[test]
    fn test_play_too_deep_position() {
        let mut grid = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let position: Vec<usize> = vec![2, 3];
        let result = grid.play(&position, Cell::Cross);
        assert!(result.is_err());

        if let Err(ApplicationError::Grid(GridError::InvalidPosition(_))) = result {
            assert!(true);
        } else {
            panic!("Expected GridError::InvalidPosition, but got a different error");
        }
    }

    #[test]
    fn test_play_not_empty_cell() {
        let mut grid = Grid {
            cells: vec![
                Cell::Empty, Cell::Circle, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let position: Vec<usize> = vec![1];
        let result = grid.play(&position, Cell::Cross);
        assert!(result.is_err());

        if let Err(ApplicationError::Cell(CellError::AlreadyUsed())) = result {
            assert!(true);
        } else {
            panic!("Expected CellError::AlreadyUsed, but got a different error");
        }
    }

    #[test]
    fn test_play() {
        let mut grid = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let position: Vec<usize> = vec![1];
        let result = grid.play(&position, Cell::Circle);
        assert!(result.is_ok());
    }

    #[test]
    fn test_play_in_subgrid() {
        let subgrid = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };
        let mut grid = Grid {
            cells: vec![
                Cell::Empty, Cell::Empty, Cell::SubGrid(subgrid),
                Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let position: Vec<usize> = vec![2, 1];
        let result = grid.play(&position, Cell::Cross);
        assert!(result.is_ok());
    }
}
