use crate::color::Color;

#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Full(Color),
}


pub struct Board {  
    rows : usize,
    columns : usize,
    cells : Vec<Cell>,
}

impl Board {
    pub fn new(rows : usize, columns : usize) -> Board {
        Board { 
            rows, 
            columns, 
            cells: vec![Cell::Empty; rows * columns]
        }
    }

    pub fn get_cell(&self, row : usize, col : usize) -> Result<Cell, String> {
        match self.is_in_bounds(row, col) {
            true => Result::Ok(self.cells[row * self.columns + col].clone()),
            false => Result::Err(format!("Outsize of bounds : {}, {}", row, col))
        }
    }

    pub fn set_cell(&mut self, cell : Cell, row : usize, col : usize) {
        if !self.is_in_bounds(row, col) {
            panic!("Trying to set a cell outside of board");
        }

        self.cells[row * self.columns + col] = cell;
    }

    pub fn is_cell_empty(&self, row : usize, col : usize) -> Result<bool, String> {
        match self.get_cell(row, col) {
            Ok(cell) => Ok(cell == Cell::Empty),
            Err(str) => Err(str)
        } 
    }

    fn is_in_bounds(&self, row:usize, col:usize) -> bool {
        self.rows > row && self.columns > col
    }
}


#[cfg(test)]
mod tests {
    use crate::{board::{Board, Cell}, color::Color};

    #[test]
    fn construct() {
        let b = Board::new(3, 2);

        assert_eq!(b.rows, 3);
        assert_eq!(b.columns, 2);
        assert_eq!(b.cells.len(), 6);
        assert!(b.cells.iter().all(|c| *c == Cell::Empty));
    }

    #[test]
    fn get_set_cells() {
        let red = Color{r:255, g:0, b:0};
        let green = Color{r:0, g:255, b:0};
        let blue = Color{r:0, g:0, b:255};

        // Should draw this pattern (R = red, G = green, B = blue, E = empty) 
        // RE
        // EG
        // BE
        let mut b = Board::new(3, 2);
        b.set_cell(Cell::Full(red.clone()), 0, 0);
        b.set_cell(Cell::Full(green.clone()), 1, 1);
        b.set_cell(Cell::Full(blue.clone()), 2, 0);

        
        assert_eq!(b.get_cell(0, 0).unwrap(), Cell::Full(red));
        assert_eq!(b.get_cell(0, 1).unwrap(), Cell::Empty);
        assert_eq!(b.get_cell(1, 0).unwrap(), Cell::Empty);
        assert_eq!(b.get_cell(1, 1).unwrap(), Cell::Full(green));
        assert_eq!(b.get_cell(2, 0).unwrap(), Cell::Full(blue));
        assert_eq!(b.get_cell(2, 1).unwrap(), Cell::Empty);

        assert_eq!(b.is_cell_empty(0, 0).unwrap(), false);
        assert_eq!(b.is_cell_empty(0, 1).unwrap(), true);
        assert_eq!(b.is_cell_empty(1, 0).unwrap(), true);
        assert_eq!(b.is_cell_empty(1, 1).unwrap(), false);
        assert_eq!(b.is_cell_empty(2, 0).unwrap(), false);
        assert_eq!(b.is_cell_empty(2, 1).unwrap(), true);

        let expected_error1 : Result<bool, String> = Err("Outsize of bounds : 3, 2".to_string());
        assert_eq!(b.is_cell_empty(3, 2), expected_error1);

        let expected_error2 : Result<bool, String> = Err("Outsize of bounds : 3, 0".to_string());
        assert_eq!(b.is_cell_empty(3, 0), expected_error2);

        let expected_error3 : Result<bool, String> = Err("Outsize of bounds : 0, 2".to_string());
        assert_eq!(b.is_cell_empty(0, 2), expected_error3);
    }
}