use super::{File, PieceType, Rank, Square};

enum CastleType {
    Short,
    Long,
}

enum Disambiguation {
    File(File),
    Rank(Rank),
    Square(Square),
}

enum BaseMove {
    Normal {
        from: Option<Disambiguation>,
        to: Square,
        piece: PieceType,
    },
    Capture {
        from: Option<Disambiguation>,
        to: Square,
        piece: PieceType,
    },
    EnPassant {
        from: File,
        to: Square,
    },
    Castle(CastleType),
    Promotion {
        to: Square,
        promote_to: PieceType,
    },
    PromotionCapture {
        from: File,
        to: Square,
        promote_to: PieceType,
    },
}

enum Move {
    Move(BaseMove),
    Check(BaseMove),
    Checkmate(BaseMove),
    Draw,
}

impl Move {
    fn parse(input: &str) -> Result<Self, String> {
        Err(String::from("..."))
    }
}

