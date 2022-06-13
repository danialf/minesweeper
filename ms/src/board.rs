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

    fn get(&self, position: Position) -> Field {
        self[position.0][position.1]
    }

    fn set(&mut self, position: Position, value: Field) {
        self[position.0][position.1] = value;
    }

    /// iterate over neighboring elements by given position
    fn iter_neighbors(&self, (rows, cols): Position) -> impl Iterator<Item = Position> + '_ {
        ((rows.max(1) - 1)..=(rows + 1).min(self.get_size().get_rows() - 1))
            .flat_map(move |i| {
                ((cols.max(1) - 1)..=(cols + 1).min(self.get_size().get_cols() - 1))
                    .map(move |j| (i, j))
            })
            .filter(move |&pos| pos != (rows, cols))
    }

    /// count number of the mines adjacent to given position
    fn count_neighboring_mines(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(|elem| match self.get(*elem) {
                Field::Mine(_) => true,
                _ => false,
            })
            .count() as u8
    }

    /// count number of the flagged field adjacent to given position
    fn count_neighboring_flags(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(|&position| match self.get(position) {
                Field::Mine(state) => match state {
                    MineState::Hidden => false,
                    MineState::Detonated => false,
                    MineState::Flagged => true,
                },
                Field::Flagged => true,
                _ => false,
            })
            .count() as u8
    }

    /// toggle flag given position
    pub fn toggle_flag(&mut self, position: Position) {
        match self.get(position) {
            Field::Closed => self.set(position, Field::Flagged),
            Field::Opened(_) => return,
            Field::Mine(state) => match state {
                MineState::Hidden => self.set(position, Field::Mine(MineState::Flagged)),
                MineState::Detonated => return,
                MineState::Flagged => self.set(position, Field::Mine(MineState::Hidden)),
            },
            Field::Flagged => self.set(position, Field::Closed),
        }
    }

    /// .open a field by given position
    pub fn open(&mut self, position: Position) {
        match self.get(position) {
            Field::Closed => {
                // open neighboring too
                let mine_count = self.count_neighboring_mines(position);

                if mine_count == 0 {
                    self.set(position, Field::Opened(0));

                    let neighbors: Vec<Position> = self.iter_neighbors(position).collect();
                    for neighbor in neighbors {
                        if self.get(neighbor) == Field::Closed {
                            self.open(neighbor);
                        }
                    }
                } else {
                    self.set(position, Field::Opened(mine_count));
                }
            }
            Field::Opened(mine_count) => {
                // marked all possible mines?
                let flag_count = self.count_neighboring_flags(position);

                if mine_count == flag_count {
                    let neighbors: Vec<Position> = self
                        .iter_neighbors(position)
                        .filter(|&pos| match self.get(pos) {
                            Field::Closed => true,
                            Field::Opened(_) => false,
                            Field::Mine(state) => match state {
                                MineState::Hidden => true,
                                MineState::Detonated => true,
                                MineState::Flagged => false,
                            },
                            Field::Flagged => false,
                        })
                        .collect();

                    for pos in neighbors {
                        self.open(pos);
                    }
                }
                return;
            }
            Field::Mine(state) => match state {
                MineState::Hidden => self.set(position, Field::Mine(MineState::Detonated)),
                MineState::Detonated => return,
                MineState::Flagged => return,
            },
            Field::Flagged => return,
        }
    }

    /// reveal all the mins
    pub fn reveal(&self) {}

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
    use crate::{field::{Field, MineState, Position}, random};

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
    fn test_board_open() {
        let mut b = Board::new(10, 10);

        // b[0][0] = Field::Opened(3);
        b[1][0] = Field::Mine(MineState::Hidden);
        b[0][1] = Field::Mine(MineState::Hidden);
        b[1][1] = Field::Mine(MineState::Hidden);

        b.open((0, 0));

        println!("{}", b);

        b.open((2, 1));
        println!("{}", b);

        b.open((3, 3));
        println!("{}", b);
    }

    #[test]
    #[ignore = "Just for functionality test"]
    fn test()
    {
        let mut board = Board::new(10,10);
        let mines = random::random_position_vec(board.get_size(), 10);
        board.plant_mines(mines);

        let mut opened_vec = vec![];

        for _ in 0..15{
            opened_vec.push(random::random_position(board.get_size()));
            board.open(*opened_vec.last().unwrap());
        }

        println!("{}",board);
        println!("{:?}", opened_vec);
    }

    #[test]
    #[ignore]
    fn test_board_display() {
        let mut b = Board::new(10, 10);

        b[0][0] = Field::Opened(3);
        b[1][0] = Field::Flagged;
        b[0][1] = Field::Mine(MineState::Hidden);
        b[0][2] = Field::Mine(MineState::Hidden);

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
    fn test_board_iter_neighbors_upper_bound() {
        let b = Board::new(5, 5);

        let neighbors: Vec<Position> = b.iter_neighbors((0, 0)).collect();

        assert_eq!(neighbors.len(), 3);

        assert_eq!(neighbors.contains(&(1, 0)), true);
        assert_eq!(neighbors.contains(&(1, 1)), true);
        assert_eq!(neighbors.contains(&(0, 1)), true);
    }

    #[test]
    fn test_board_iter_neighbors_lower_bound() {
        let b = Board::new(5, 5);

        let neighbors: Vec<Position> = b.iter_neighbors((4, 4)).collect();

        assert_eq!(neighbors.len(), 3);

        assert_eq!(neighbors.contains(&(4, 3)), true);
        assert_eq!(neighbors.contains(&(3, 3)), true);
        assert_eq!(neighbors.contains(&(3, 4)), true);
    }
}
