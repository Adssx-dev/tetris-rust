use crate::color::Color;

#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Full(Color),
}


struct Board {  
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

    pub fn get_cell(&self, row : usize, col : usize) -> Cell{
        self.cells[row * self.columns + col].clone()
    }

    pub fn set_cell(&mut self, cell : Cell, row : usize, col : usize) {
        self.cells[row * self.columns + col] = cell;
    }

    pub fn is_cell_empty(&self, row : usize, col : usize) -> bool {
        self.get_cell(row, col) == Cell::Empty
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

        
        assert_eq!(b.get_cell(0, 0), Cell::Full(red));
        assert_eq!(b.get_cell(0, 1), Cell::Empty);
        assert_eq!(b.get_cell(1, 0), Cell::Empty);
        assert_eq!(b.get_cell(1, 1), Cell::Full(green));
        assert_eq!(b.get_cell(2, 0), Cell::Full(blue));
        assert_eq!(b.get_cell(2, 1), Cell::Empty);

        assert_eq!(b.is_cell_empty(0, 0), false);
        assert_eq!(b.is_cell_empty(0, 1), true);
        assert_eq!(b.is_cell_empty(1, 0), true);
        assert_eq!(b.is_cell_empty(1, 1), false);
        assert_eq!(b.is_cell_empty(2, 0), false);
        assert_eq!(b.is_cell_empty(2, 1), true);
    }
}