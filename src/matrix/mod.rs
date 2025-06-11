pub mod vector;
use crate::matrix::vector::Vector;

#[derive(Debug, Default)]
pub struct Matrix {
    vector: [Vector; 3],
}

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> &Option<bool> {
        self.vector[row].get(col)
    }

    pub fn get_row(&self, row: usize) -> &Vector {
        &self.vector[row]
    }

    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        self.vector[row].set(col, value);
    }

    pub fn win_condition(&self, row: usize, col: usize, value: bool) -> bool {
        const WIN: bool = true;
        const LOSE: bool = false;

        // row
        self.get_row(row).vector.iter().all(|v| {
            matches!(v, Some(val) if *val == value)
        });

        // column check
        {
            if !self.get(row, col).eq(&Some(value)) {
                LOSE
            } else {
                WIN
            }
        }



        // dia

    }
}
