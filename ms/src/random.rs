use rand::{thread_rng, Rng};

use crate::{field::Position, size::Size};

pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();

    rng.gen_range(min..max)
}

pub fn random_position(size: Size) -> Position {
    (
        (random_range(0, size.get_rows())),
        (random_range(0, size.get_cols())),
    )
}

pub fn random_position_vec(size: Size, total: usize) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::with_capacity(total);

    while positions.len() < total {
        let pos = random_position(size);

        if positions.contains(&pos) {
            continue;
        }

        positions.push(pos);
    }
    positions
}
