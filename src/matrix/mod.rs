pub mod vector;
use crate::matrix::vector::Vector;
use std::fmt;

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

    pub fn check_win(&self, row: usize, col: usize, value: bool) -> bool {
        let check_row = self.get_row(row).vector.iter().all(|v| {   // row check
            matches!(v, Some(val) if *val == value)
        });

        let vertical = [*self.vector[0].get(col), *self.vector[1].get(col), *self.vector[2].get(col)];  // col check
        let check_col = general_win_checker(vertical, value);

        let dia_pos = [*self.vector[0].get(0), *self.vector[1].get(1), *self.vector[2].get(2)]; // positive dia check
        let check_dia_pos = general_win_checker(dia_pos, value);

        let dia_neg = [*self.vector[0].get(2), *self.vector[1].get(1), *self.vector[2].get(0)]; // negative dia check
        let check_dia_neg = general_win_checker(dia_neg, value);

        check_row || check_col || check_dia_neg || check_dia_pos
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for(i, row) in self.vector.iter().enumerate() {
            for(j, cell) in row.vector.iter().enumerate() {
                let symbol = match cell {
                    Some(true) => "X",
                    Some(false) => "O",
                    None => " ",
                };
                write!(f, "{}", symbol)?;
                if j < 2 { write!(f, "|")?; }
            }
            writeln!(f)?;
            if i < 2 {writeln!(f, "---|---|---")?;  }
        }
        Ok(())
    }
}

fn general_win_checker(arr:[Option<bool>; 3], val:bool) -> bool {
    arr
        .iter()
        .all(|el| matches!(el, Some(el) if *el == val))
}

#[cfg(test)]
mod tests {
    use crate::matrix::{general_win_checker, Matrix};

    #[test]
    fn new_matrix_should_have_all_empty_positions() {
        let m = Matrix::default();
        for el in m.vector.iter() {
            if el.vector[0] != None { return assert!(false); }
        }
        assert!(true);
    }

    #[test]
    fn should_matrix_get_return_right_value() {
        //let m = Matrix::default();

    }

    #[test]
    fn should_check_row_win_return_true() {
        let mut m_false = Matrix::default();
        m_false.vector[0].set(0, false);
        m_false.vector[0].set(1, false);
        m_false.vector[0].set(2, false);
        let mut m_true = Matrix::default();
        m_true.vector[1].set(0, true);
        m_true.vector[1].set(1, true);
        m_true.vector[1].set(2, true);
        assert!(m_false.check_win(0, 0, false));
        assert!(m_true.check_win(1, 1, true));
    }

    #[test]
    fn should_check_row_win_return_false() {
        let mut m_false = Matrix::default();
        m_false.vector[0].set(0, false);
        m_false.vector[0].set(1, false);
        m_false.vector[0].set(2, true);
        let mut m_true = Matrix::default();
        m_true.vector[1].set(0, false);
        m_true.vector[1].set(1, true);
        m_true.vector[1].set(2, true);
        assert!(!m_false.check_win(0, 2, false));
        assert!(!m_true.check_win(1, 0, true));
    }

    #[test]
    fn should_check_col_win_return_true() {
        let mut m_false = Matrix::default();
        m_false.vector[0].set(0, false);
        m_false.vector[1].set(0, false);
        m_false.vector[2].set(0, false);
        let mut m_true = Matrix::default();
        m_true.vector[0].set(1, true);
        m_true.vector[1].set(1, true);
        m_true.vector[2].set(1, true);
        assert!(m_false.check_win(2, 0, false));
        assert!(m_true.check_win(2, 1, true));
    }

    #[test]
    fn should_check_col_win_return_false() {
        let mut m_false = Matrix::default();
        m_false.vector[0].set(0, true);
        m_false.vector[1].set(0, false);
        m_false.vector[2].set(0, false);
        let mut m_true = Matrix::default();
        m_true.vector[0].set(1, true);
        m_true.vector[1].set(1, false);
        m_true.vector[2].set(1, true);
        assert!(!m_false.check_win(2, 0, false));
        assert!(!m_true.check_win(2, 1, true));
    }

    #[test]
    fn should_check_diagonals_win_return_true() {
        let mut m = Matrix::default();
        m.vector[0].set(0, false);
        m.vector[1].set(0, false);
        m.vector[2].set(0, false);
        assert!(m.check_win(2, 0, false));

        let mut m2 = Matrix::default();
        m2.vector[0].set(2, true);
        m2.vector[1].set(1, true);
        m2.vector[2].set(0, true);
        assert!(m2.check_win(1, 1, true));
    }

    #[test]
    fn should_check_diagonals_win_return_false() {
        let mut m = Matrix::default();
        m.vector[0].set(0, false);
        m.vector[1].set(0, false);
        m.vector[2].set(0, true);
        assert!(!m.check_win(2, 0, true));

        let mut m2 = Matrix::default();
        m2.vector[0].set(2, true);
        m2.vector[1].set(1, false);
        m2.vector[2].set(0, true);
        assert!(!m2.check_win(1, 1, false));
    }

    #[test]
    fn general_win_checker_true() {
        let arr_true = [Some(true), Some(true), Some(true)];
        let arr_false = [Some(false), Some(false), Some(false)];
        assert!(general_win_checker(arr_true, true));
        assert!(general_win_checker(arr_false, false));
    }

    #[test]
    fn general_win_checker_false() {
        let arr_1 = [Some(true), Some(false), Some(false)];
        let arr_2 = [Some(false), Some(true), Some(false)];
        let arr_3 = [Some(false), Some(false), Some(true)];
        assert!(!general_win_checker(arr_1, false));
        assert!(!general_win_checker(arr_2, false));
        assert!(!general_win_checker(arr_3, false));
    }
}
