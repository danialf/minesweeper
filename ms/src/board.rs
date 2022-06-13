use std::{
    fmt::{Display, Write},
    ops::{Index, IndexMut},
};

use crate::{
    field::{Field, MineState, Position},
    size::Size,
};

pub struct Board {
    size: Size,
    fields: Box<[Field]>,
}

impl Board {
    pub fn new(cols: usize, rows: usize) -> Self {
        let size = Size::new(cols, rows);
        let elems = vec![Field::Closed; size.total()];

        Board {
            size,
            fields: { elems.into_boxed_slice() },
        }
    }

    fn iter_neighbors<'a>(&'a self, (rows, cols): Position) -> impl Iterator<Item = Position> + 'a {
        ((rows.max(1) - 1)..=(rows + 1).min(self.get_size().get_rows() - 1))
            .flat_map(move |i| {
                ((cols.max(1) - 1)..=(cols + 1).min(self.get_size().get_cols() - 1))
                    .map(move |j| (i, j))
            })
            .filter(move |&pos| pos != (rows, cols))
    }

    pub fn reset(&mut self) {
        for i in 0..self.fields.len() {
            self.fields[i] = Field::Closed;
        }
    }

    pub fn plant_mines(&mut self, positions: Vec<Position>) {
        for (rows, cols) in positions {
            self[rows][cols] = Field::Mine(MineState::Hidden)
        }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rows in 0..self.size.get_rows() {
            for cols in 0..self.get_size().get_cols() {
                write!(f, "{}", self[rows][cols])?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Index<usize> for Board {
    type Output = [Field];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.get_size().get_cols();
        let end = (index + 1) * self.get_size().get_cols();

        &self.fields[start..end]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut [Field] {
        let start = index * self.get_size().get_cols();
        let end = (index + 1) * self.get_size().get_cols();

        &mut self.fields[start..end]
    }
}

#[cfg(test)]
mod tests {
    use crate::field::{Field, MineState, Position};

    use super::Board;

    #[test]
    fn test_board_2d_indexer() {
        let mut b = Board::new(10, 10);

        b[0][0] = Field::Opened(3);
        b[1][0] = Field::Flagged;
        b[0][1] = Field::Mine(MineState::Hidden);

        assert_eq!(b[0][0], Field::Opened(3));
        assert_eq!(b[1][0], Field::Flagged);
        assert_eq!(b[0][1], Field::Mine(MineState::Hidden));
    }

    #[test]
    #[ignore]
    fn test_board_display() {
        let mut b = Board::new(10, 10);

        b[0][0] = Field::Opened(3);
        b[1][0] = Field::Flagged;
        b[0][1] = Field::Mine(MineState::Hidden);

        b[9][9] = Field::Mine(MineState::Detonated);
        println!("{}", b);
    }

    #[test]
    fn test_board_reset() {
        let mut b = Board::new(5, 5);

        b[0][0] = Field::Mine(MineState::Hidden);
        b[1][1] = Field::Flagged;
        b[1][3] = Field::Flagged;
        b[1][4] = Field::Flagged;

        b.reset();

        b.fields.into_iter().for_each(|item| {
            assert_eq!(*item, Field::Closed);
        });
    }

    #[test]
    fn test_board_plant_mines() {
        let mut b = Board::new(5, 5);

        let vec: Vec<Position> = vec![(1, 1), (1, 2), (2, 2), (4, 4)];

        b.plant_mines(vec);

        assert_eq!(b[1][1], Field::Mine(MineState::Hidden));
        assert_eq!(b[1][2], Field::Mine(MineState::Hidden));
        assert_eq!(b[2][2], Field::Mine(MineState::Hidden));
        assert_eq!(b[4][4], Field::Mine(MineState::Hidden));

        assert_eq!(b[0][0], Field::Closed);
    }

    #[test]
    fn test_board_iter_neighbors_upper_bound()
    {
        let b = Board::new(5,5);
        
        let neighbors: Vec<Position> = b.iter_neighbors((0,0)).collect();

        assert_eq!(neighbors.len(),3);

        assert_eq!(neighbors.contains(&(1,0)),true);
        assert_eq!(neighbors.contains(&(1,1)),true);
        assert_eq!(neighbors.contains(&(0,1)),true);
    }

    #[test]
    fn test_board_iter_neighbors_lower_bound()
    {
        let b = Board::new(5,5);
        
        let neighbors: Vec<Position> = b.iter_neighbors((4,4)).collect();

        assert_eq!(neighbors.len(),3);

        assert_eq!(neighbors.contains(&(4,3)),true);
        assert_eq!(neighbors.contains(&(3,3)),true);
        assert_eq!(neighbors.contains(&(3,4)),true);
    }
}
