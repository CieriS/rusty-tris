#[derive(Debug, Default)]
pub struct Vector {
    pub vector: [Option<bool>; 3],
}


impl Vector {
    pub fn get(&self, el: usize) -> &Option<bool> {
        &self.vector[el]
    }

    pub fn set(&mut self, el: usize, value: bool) {
        self.vector[el] = Some(value)
    }
}
