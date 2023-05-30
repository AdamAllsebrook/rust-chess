use super::{File, PieceType, Rank, Square};

#[derive(PartialEq, Eq)]
enum CastleType {
    Short,
    Long,
}

#[derive(PartialEq, Eq)]
enum Disambiguation {
    File(File),
    Rank(Rank),
    Square(Square),
}

#[derive(PartialEq, Eq)]
enum MoveType {
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

#[derive(PartialEq, Eq)]
enum GameResult {
    WhiteWins,
    BlackWins,
    Draw,
}

#[derive(PartialEq, Eq)]
enum Move {
    NoCheck(MoveType),
    Check(MoveType),
    Checkmate(MoveType),
    DrawOffer,
    EndOfGame(GameResult),
}

// A symbol for one character of the input
enum InputToken {
    Piece(PieceType),
    File(File),
    Rank(Rank),
    Capture,
    Promotion,
    Check,
    Checkmate,
}

#[derive(PartialEq, Eq)]
enum MoveParseError {
    NotAMove,
}

impl Move {
    fn parse(input: &str) -> Result<Self, MoveParseError> {
        match input {
            "O-O" => Ok(Move::NoCheck(MoveType::Castle(CastleType::Short))),
            "O-O-O" => Ok(Move::NoCheck(MoveType::Castle(CastleType::Long))),
            "(=)" => Ok(Move::DrawOffer),
            "1-0" => Ok(Move::EndOfGame(GameResult::WhiteWins)),
            "0-1" => Ok(Move::EndOfGame(GameResult::BlackWins)),
            "1/2-1/2" => Ok(Move::EndOfGame(GameResult::Draw)),

            _ => match Move::tokenize(input) {
                Ok(input_tokens) => match input_tokens.last() {
                    Some(InputToken::Check) => {
                        Move::parse_move_type(&input_tokens[..input_tokens.len() - 1])
                            .map(Move::Check)
                    }
                    Some(InputToken::Checkmate) => {
                        Move::parse_move_type(&input_tokens[..input_tokens.len() - 1])
                            .map(Move::Checkmate)
                    }
                    _ => Move::parse_move_type(&input_tokens).map(Move::NoCheck),
                },
                Err(e) => Err(e),
            },
        }
    }
    fn parse_move_type(input_parts: &[InputToken]) -> Result<MoveType, MoveParseError> {
        use InputToken::*;
        match input_parts[..] {
            [File(file), Rank(rank)] => Ok(MoveType::Normal {
                from: None,
                to: Square::new(file, rank),
                piece: PieceType::Pawn,
            }),
            [Piece(piece), File(file), Rank(rank)] => Ok(MoveType::Normal {
                from: None,
                to: Square::new(file, rank),
                piece,
            }),
            [Piece(piece), File(from_file), File(file), Rank(rank)] => Ok(MoveType::Normal {
                from: Some(Disambiguation::File(from_file)),
                to: Square::new(file, rank),
                piece,
            }),
            [Piece(piece), Rank(from_rank), File(file), Rank(rank)] => Ok(MoveType::Normal {
                from: Some(Disambiguation::Rank(from_rank)),
                to: Square::new(file, rank),
                piece,
            }),
            [Piece(piece), File(from_file), Rank(from_rank), File(file), Rank(rank)] => {
                Ok(MoveType::Normal {
                    from: Some(Disambiguation::Square(Square::new(from_file, from_rank))),
                    to: Square::new(file, rank),
                    piece,
                })
            }
            [Piece(piece), Capture, File(file), Rank(rank)] => Ok(MoveType::Capture {
                from: None,
                to: Square::new(file, rank),
                piece,
            }),
            [Piece(piece), File(from_file), Capture, File(file), Rank(rank)] => {
                Ok(MoveType::Capture {
                    from: Some(Disambiguation::File(from_file)),
                    to: Square::new(file, rank),
                    piece,
                })
            }
            [Piece(piece), Rank(from_rank), Capture, File(file), Rank(rank)] => {
                Ok(MoveType::Capture {
                    from: Some(Disambiguation::Rank(from_rank)),
                    to: Square::new(file, rank),
                    piece,
                })
            }
            [Piece(piece), File(from_file), Rank(from_rank), Capture, File(file), Rank(rank)] => {
                Ok(MoveType::Capture {
                    from: Some(Disambiguation::Square(Square::new(from_file, from_rank))),
                    to: Square::new(file, rank),
                    piece,
                })
            }
            [File(from_file), Capture, File(file), Rank(rank)] => Ok(MoveType::Capture {
                from: Some(Disambiguation::File(from_file)),
                to: Square::new(file, rank),
                piece: PieceType::Pawn,
            }),
            [File(file), Rank(rank), Promotion, Piece(promote_to)] => Ok(MoveType::Promotion {
                to: Square::new(file, rank),
                promote_to,
            }),
            [File(from_file), Capture, File(file), Rank(rank), Promotion, Piece(promote_to)] => {
                Ok(MoveType::PromotionCapture {
                    from: from_file,
                    to: Square::new(file, rank),
                    promote_to,
                })
            }
            _ => Err(MoveParseError::NotAMove),
        }
    }

    fn tokenize(input: &str) -> Result<Vec<InputToken>, MoveParseError> {
        input
            .chars()
            .map(|c| match c {
                'x' => Ok(InputToken::Capture),
                '=' => Ok(InputToken::Promotion),
                '+' => Ok(InputToken::Check),
                '#' => Ok(InputToken::Checkmate),
                _ => {
                    if let Some(piece) = PieceType::from_char(c) {
                        Ok(InputToken::Piece(piece))
                    } else if let Some(file) = File::from_char(c) {
                        Ok(InputToken::File(file))
                    } else if let Some(rank) = Rank::from_char(c) {
                        Ok(InputToken::Rank(rank))
                    } else {
                        Err(MoveParseError::NotAMove)
                    }
                }
            })
            .collect()
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
