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

#[derive(PartialEq, Eq)]
#[derive(PartialEq, Eq)]
enum MoveParseError {
    NotAMove,
}

impl Move {
    fn parse(input: &str) -> Result<Self, String> {
        Err(String::from("..."))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{super::square, Move::*, MoveType::*, PieceType::*, *};

    // valid tests
    #[test]
    fn Be5() {
        let parsed = Move::parse("Be5");
        let expected = NoCheck(Normal {
            from: None,
            to: square!(E 5),
            piece: Bishop,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn Nf3() {
        let parsed = Move::parse("Nf3");
        let expected = NoCheck(Normal {
            from: None,
            to: square!(F 3),
            piece: Knight,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn c5() {
        let parsed = Move::parse("c5");
        let expected = NoCheck(Normal {
            from: None,
            to: square!(C 5),
            piece: Pawn,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn Bxe5() {
        let parsed = Move::parse("Bxe5");
        let expected = NoCheck(Capture {
            from: None,
            to: square!(E 5),
            piece: Bishop,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn exd6() {
        let parsed = Move::parse("exd6");
        let expected = NoCheck(Capture {
            from: Some(Disambiguation::File(File::E)),
            to: square!(D 6),
            piece: Pawn,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn Rdf8() {
        let parsed = Move::parse("Rdf8");
        let expected = NoCheck(Normal {
            from: Some(Disambiguation::File(File::D)),
            to: square!(F 8),
            piece: Rook,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn R1a3() {
        let parsed = Move::parse("R1a3");
        let expected = NoCheck(Normal {
            from: Some(Disambiguation::Rank(Rank::One)),
            to: square!(A 3),
            piece: Rook,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn Qh4e1() {
        let parsed = Move::parse("Qh4e1");
        let expected = NoCheck(Normal {
            from: Some(Disambiguation::Square(square!(H 4))),
            to: square!(E 1),
            piece: Queen,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn Rdxf8() {
        let parsed = Move::parse("Rdxf8");
        let expected = NoCheck(Capture {
            from: Some(Disambiguation::File(File::D)),
            to: square!(F 8),
            piece: Rook,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn R1xa3() {
        let parsed = Move::parse("R1xa3");
        let expected = NoCheck(Capture {
            from: Some(Disambiguation::Rank(Rank::One)),
            to: square!(A 3),
            piece: Rook,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn Qh4xe1() {
        let parsed = Move::parse("Qh4xe1");
        let expected = NoCheck(Capture {
            from: Some(Disambiguation::Square(square!(H 4))),
            to: square!(E 1),
            piece: Queen,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn e8_eq_Q() {
        let parsed = Move::parse("e8=Q");
        let expected = NoCheck(Promotion {
            to: square!(E 8),
            promote_to: Queen,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn gxf8_eq_N() {
        let parsed = Move::parse("gxf8=N");
        let expected = NoCheck(PromotionCapture {
            from: File::G,
            to: square!(F 8),
            promote_to: Knight,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn draw_offer() {
        let parsed = Move::parse("(=)");
        let expected = DrawOffer;
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn short_castle() {
        let parsed = Move::parse("O-O");
        let expected = NoCheck(Castle(CastleType::Short));
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn long_castle() {
        let parsed = Move::parse("O-O-O");
        let expected = NoCheck(Castle(CastleType::Long));
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn check() {
        let parsed = Move::parse("Qh4+");
        let expected = Check(Normal {
            from: None,
            to: square!(H 4),
            piece: Queen,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn checkmate() {
        let parsed = Move::parse("Qh4#");
        let expected = Checkmate(Normal {
            from: None,
            to: square!(H 4),
            piece: Queen,
        });
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn white_wins() {
        let parsed = Move::parse("1-0");
        let expected = EndOfGame(GameResult::WhiteWins);
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn black_wins() {
        let parsed = Move::parse("0-1");
        let expected = EndOfGame(GameResult::BlackWins);
        assert!(parsed == Ok(expected))
    }

    #[test]
    fn draw() {
        let parsed = Move::parse("1/2-1/2");
        let expected = EndOfGame(GameResult::Draw);
        assert!(parsed == Ok(expected))
    }

    // invalid tests
    #[test]
    fn empty() {
        let parsed = Move::parse("");
        assert!(parsed == Err(MoveParseError::NotAMove))
    }

    #[test]
    fn invalid_move() {
        let parsed = Move::parse("invalid");
        assert!(parsed == Err(MoveParseError::NotAMove))
    }

    #[test]
    fn not_a_square() {
        let parsed = Move::parse("Qh9");
        assert!(parsed == Err(MoveParseError::NotAMove))
    }

    #[test]
    fn not_a_piece() {
        let parsed = Move::parse("Jf3");
        assert!(parsed == Err(MoveParseError::NotAMove))
    }
}
