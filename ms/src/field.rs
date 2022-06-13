use std::fmt::Display;

const CLOSED: &str = "🟦 ";
const OPENED: &str = "⬜ ";
const MINE: &str = "💣 ";
const FLAG: &str = "🚩 ";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MineState {
    Hidden,
    Detonated,
    Flagged,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Field {
    Closed,
    Opened(u8),
    Mine(MineState),
    Flagged,
}

pub type Position = (usize,usize);

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Closed => f.write_str(CLOSED),
            Field::Opened(n) => {
                if n > &0 {
                    write!(f, " {} ", n)
                } else {
                    f.write_str(OPENED)
                }
            }
            Field::Mine(state) => match state {
                MineState::Hidden => f.write_str(CLOSED),
                MineState::Detonated => f.write_str(MINE),
                MineState::Flagged => f.write_str(FLAG),
            },
            Field::Flagged => f.write_str(FLAG),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::field::{Field, MineState, CLOSED, FLAG, MINE, OPENED};

    #[test]
    fn test_field_display() {
        assert_eq!(Field::Closed.to_string(), CLOSED);
        assert_eq!(Field::Opened(0).to_string(), OPENED);
        assert_eq!(Field::Opened(3).to_string(), " 3 ");
        assert_eq!(Field::Mine(MineState::Hidden).to_string(), CLOSED);
        assert_eq!(Field::Mine(MineState::Flagged).to_string(), FLAG);
        assert_eq!(Field::Mine(MineState::Detonated).to_string(), MINE);
        assert_eq!(Field::Flagged.to_string(), FLAG);
    }
}
