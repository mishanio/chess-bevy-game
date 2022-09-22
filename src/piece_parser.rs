use std::collections::HashMap;

use crate::models::chess_models::{ChessColor, ChessPiece, PieceType};

pub struct PieceParser;

impl PieceParser {
    fn mappings<'a>() -> HashMap<&'a str, (ChessColor, PieceType)> {
        return HashMap::from([
            ("b_pa", (ChessColor::BLACK, PieceType::PAWN)),
            ("b_ro", (ChessColor::BLACK, PieceType::ROOK)),
            ("b_kn", (ChessColor::BLACK, PieceType::KNIGHT)),
            ("b_bi", (ChessColor::BLACK, PieceType::BISHOP)),
            ("b_ki", (ChessColor::BLACK, PieceType::KING)),
            ("b_qu", (ChessColor::BLACK, PieceType::QUEEN)),
            ("w_pa", (ChessColor::WHITE, PieceType::PAWN)),
            ("w_ro", (ChessColor::WHITE, PieceType::ROOK)),
            ("w_kn", (ChessColor::WHITE, PieceType::KNIGHT)),
            ("w_bi", (ChessColor::WHITE, PieceType::BISHOP)),
            ("w_ki", (ChessColor::WHITE, PieceType::KING)),
            ("w_qu", (ChessColor::WHITE, PieceType::QUEEN)),
        ]);
    }

    pub fn default_map() -> String {
        let string = "|b_ro|b_kn|b_bi|b_ki|b_qu|b_bi|b_kn|b_ro|\n
                            |b_pa|b_pa|b_pa|b_pa|b_pa|b_pa|b_pa|b_pa|\n
                            |none|none|none|none|none|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |w_pa|w_pa|w_pa|w_pa|w_pa|w_pa|w_pa|w_pa|\n
                            |w_ro|w_kn|w_bi|w_ki|w_qu|w_bi|w_kn|w_ro|\n
                            ";
        return string.to_string();
    }

    pub fn parse_map(map: String) -> Vec<Option<ChessPiece>> {
        return map
            .split('\n')
            .filter(|l| !l.trim().is_empty())
            .rev()
            .enumerate()
            .flat_map(move |(j, line)| {
                line.split('|')
                    .filter(|l| !l.trim().is_empty())
                    .enumerate()
                    .map(move |(i, symbol)| PieceParser::parse_piece(symbol, i, j))
            })
            .collect();
    }

    fn parse_piece(symbol: &str, i: usize, j: usize) -> Option<ChessPiece> {
        if symbol.eq("none") {
            return None;
        }
        let mappings = PieceParser::mappings();
        return mappings.get(symbol).map(|(color, piece_type)| {
            ChessPiece::new(i as i8, j as i8, color.clone(), piece_type.clone())
        });
    }
}

#[cfg(test)]
mod run_tests {
    use crate::models::common_resources::CellPosition;

    use super::*;

    #[test]
    fn test_parse_map() {
        let result = PieceParser::parse_map(PieceParser::default_map());
        assert_eq!(64, result.len());
        for p in &result[0..16] {
            assert!(p.is_some())
        }
        for p in &result[16..48] {
            assert!(p.is_none())
        }
        for p in &result[48..64] {
            assert!(p.is_some())
        }
    }
    #[test]
    fn test_parse_piece() {
        let result = PieceParser::parse_piece("w_bi", 0, 1);
        assert!(result.is_some());
        let chess_piece = result.unwrap();

        assert_eq!(PieceType::BISHOP, chess_piece.piece_type);
        assert_eq!(ChessColor::WHITE, chess_piece.color);
        assert_eq!(CellPosition { i: 0, j: 1 }, chess_piece.pos);
    }
}
