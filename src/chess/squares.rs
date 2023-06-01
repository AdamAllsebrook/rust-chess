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

impl File {
    pub fn from_char(c: char) -> Option<File> {
        match c {
            'a' => Some(File::A),
            'b' => Some(File::B),
            'c' => Some(File::C),
            'd' => Some(File::D),
            'e' => Some(File::E),
            'f' => Some(File::F),
            'g' => Some(File::G),
            'h' => Some(File::H),
            _ => None,
        }
    }

    pub fn from_index(i: usize) -> Option<File> {
        match i {
            0 => Some(File::A),
            1 => Some(File::B),
            2 => Some(File::C),
            3 => Some(File::D),
            4 => Some(File::E),
            5 => Some(File::F),
            6 => Some(File::G),
            7 => Some(File::H),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
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

impl Rank {
    pub fn from_char(c: char) -> Option<Rank> {
        match c {
            '1' => Some(Rank::One),
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            _ => None,
        }
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

    pub fn is_light_square(&self) -> bool {
        (self.file_index + self.rank_index) % 2 == 0
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
    ($file:ident $rank_num:literal) => {
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
