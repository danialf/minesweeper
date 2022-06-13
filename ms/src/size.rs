/// Size struct
#[derive(Clone, Copy)]
pub struct Size(usize, usize);

impl Size {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self(cols, rows)
    }

    pub fn get_rows(&self) -> usize {
        self.0
    }

    pub fn get_cols(&self) -> usize {
        self.1
    }

    pub fn total(&self) -> usize {
        self.get_cols() * self.get_rows()
    }
}
