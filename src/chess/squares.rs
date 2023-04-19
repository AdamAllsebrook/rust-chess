use std::fmt;

use enum_iterator::{all, cardinality, Sequence};

#[derive(Debug, Sequence, Hash, Eq, PartialEq, Copy, Clone)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Sequence, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            all::<Rank>().position(|r| r == *self).unwrap() + 1
        )
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Square {
    rank: Rank,
    file: File,
    file_index: usize,
    rank_index: usize,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Square {
        Square {
            file,
            rank,
            file_index: all::<File>().position(|f| f == file).unwrap(),
            rank_index: all::<Rank>().position(|r| r == rank).unwrap(),
        }
    }

    pub fn from_index(file_index: usize, rank_index: usize) -> Square {
        if file_index >= cardinality::<File>() || rank_index >= cardinality::<Rank>() {
            panic!("Tried to create a Square with an out of bounds index")
        }
        Square {
            file: all::<File>().collect::<Vec<_>>()[file_index],
            rank: all::<Rank>().collect::<Vec<_>>()[rank_index],
            file_index,
            rank_index,
        }
    }

    // Use getters to ensure that the values cannot be changed,
    // violating the in-bounds guarantee of using Rank/File enums
    pub fn get_file_index(&self) -> usize {
        self.file_index
    }

    pub fn get_rank_index(&self) -> usize {
        self.rank_index
    }
}

// Use getters for file and rank indices to keep the index-in-bounds safety of using enums
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

#[macro_export]
macro_rules! square {
    ($file:ident, $rank_num:literal) => {
        Square::new(
            File::$file,
            match ($rank_num) {
                1 => Rank::One,
                2 => Rank::Two,
                3 => Rank::Three,
                4 => Rank::Four,
                5 => Rank::Five,
                6 => Rank::Six,
                7 => Rank::Seven,
                8 => Rank::Eight,
                _ => panic!("Invalid rank number"),
            },
        )
    };
}
pub(crate) use square;
