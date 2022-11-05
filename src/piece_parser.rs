use std::collections::HashMap;

use crate::models::{
    chess_piece::{ChessPiece, PieceType},
    common_chess::ChessColor, common_resources::{Board},
};

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

    fn reverse_color_mappings<'a>() -> HashMap<ChessColor,  &'a str> {
        return HashMap::from([
            (ChessColor::BLACK, "b"),
            (ChessColor::WHITE,"w"),
        ]);
    }
    fn reverse_type_mappings<'a>() -> HashMap<PieceType, &'a str> {
        return HashMap::from([
            (PieceType::PAWN, "pa"),
            (PieceType::ROOK, "ro"),
            (PieceType::KNIGHT,"kn"),
            (PieceType::BISHOP,"bi"),
            (PieceType::KING,"ki"),
            (PieceType::QUEEN,"qu"),
        ]);
    }

    pub fn default_tile_map() -> String {
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

    pub fn test_tile_map() -> String {
        let string = "|b_ro|b_kn|b_bi|b_ki|b_qu|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |none|none|none|none|none|none|none|none|\n
                            |w_ro|w_kn|w_bi|w_ki|w_qu|none|none|none|\n
                            ";
        return string.to_string();
    }

    pub fn parse_tile_map(map: String) -> Vec<Option<ChessPiece>> {
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

    pub fn save_tile_map(tiles: &Vec<&ChessPiece>, board: &Board) -> String {
        let color_mappings = PieceParser::reverse_color_mappings();
        let type_mappings = PieceParser::reverse_type_mappings();
        let mut tile_map_builder = Vec::new();
        for j in board.cell_range() {
            let mut line_builder = String::new();
            line_builder.push('|');
            for i in board.cell_range() {
               let symbol = match  tiles.iter().find(|cp| cp.pos.i == i && cp.pos.j ==j) {
                    Some(piece) => format!("{}_{}", color_mappings.get(&piece.color).unwrap(), type_mappings.get(&piece.piece_type).unwrap()),
                    None => "none".to_string(),
                };
                line_builder.push_str(symbol.as_str());
                line_builder.push('|');
            }
            line_builder.push('\n');
            tile_map_builder.push(line_builder)
        }
        let result: Vec<String> = tile_map_builder.iter().rev().map(|s| s.to_owned()).collect();
        return result.join("");
    }
}

#[cfg(test)]
mod run_tests {
    use crate::models::common_resources::CellPosition;

    use super::*;

    #[test]
    fn test_parse_map() {
        let result = PieceParser::parse_tile_map(PieceParser::default_tile_map());
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
