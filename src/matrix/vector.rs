#[derive(Debug, Default)]
pub struct Vector {
    pub vector: [Option<bool>; 3],
}


impl Vector {
    pub fn get(&self, el: usize) -> &Option<bool> {
        &self.vector[el]
    }

    pub fn set(&mut self, el: usize, value: bool) {
        let vec = self.vector[el];
        self.vector[el] = if vec==None { Some(value) } else { vec };
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::vector::Vector;

    #[test]
    fn new_vector_should_have_all_empty_positions() {
        let v = Vector::default();
        assert!(v
            .vector
            .iter()
            .all(|el| el.is_none()));
    }

    #[test]
    fn should_set_position_to_true() {
        let mut v = Vector::default();
        v.set(0, true);
        assert_eq!(v.vector[0], Some(true));
        assert_eq!(v.vector[1], None);
        assert_eq!(v.vector[2], None);
    }

    #[test]
    fn should_set_position_to_false() {
        let mut v = Vector::default();
        v.set(0, false);
        assert_eq!(v.vector[0], Some(false));
        assert_eq!(v.vector[1], None);
        assert_eq!(v.vector[2], None);
    }

    #[test]
    fn should_ignore_overwriting_position() {
        let mut v = Vector::default();
        v.set(0, false);
        v.set(0, true);
        assert_eq!(v.vector[0], Some(false));
    }

    #[test]
    fn should_reject_invalid_position() {
        // TODO: ancora non conosco la gestione degli errori
        //let mut v = Vector::default();
        //let result = v.set(v.vector.len()+1, false);
        //assert!(result.is_err(), "Invalid position");
    }
}
