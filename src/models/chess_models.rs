use bevy::{prelude::*, utils::HashMap};

use crate::models::common_resources::{Board, CellPosition};

#[derive(Default)]
pub struct MoveState {
    pub selected_piece: Option<Entity>,
    pub selected_cell: Option<Entity>,
    pub move_in_action: bool,
}

impl MoveState {
    pub fn reset(&mut self) {
        self.move_in_action = false;
        self.selected_cell = None;
        self.selected_piece = None;
    }
}

#[derive(Component)]
pub struct ChessCell {
    pub pos: CellPosition,
}
impl ChessCell {
    pub fn from(i: i8, j: i8) -> ChessCell {
        let pos = CellPosition { i, j };
        ChessCell { pos }
    }
}
#[derive(PartialEq)]
pub enum ChessColor {
    WHITE,
    BLACK,
}

pub enum PieceType {
    PAWN,
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    KING,
}

#[derive(Component)]
pub struct ChessPiece {
    pub pos: CellPosition,
    pub color: ChessColor,
    pub piece_type: PieceType,
}

impl ChessPiece {
    pub fn new(i: i8, j: i8, color: ChessColor, piece_type: PieceType) -> ChessPiece {
        let pos = CellPosition { i, j };
        ChessPiece {
            pos,
            color,
            piece_type,
        }
    }

    pub fn get_available_cells_for_move(
        &self,
        board: &Board,
        pieces: &Vec<&ChessPiece>,
    ) -> Vec<CellPosition> {
        return match self.piece_type {
            PieceType::PAWN => {
                return self.available_cells_for_pawn(board, pieces);
            }
            PieceType::ROOK => {
                return vec![];
                // return vec![cell_1, cell_2];
            }
            _ => vec![],
        };
    }

    fn available_cells_for_pawn(
        &self,
        board: &Board,
        pieces: &Vec<&ChessPiece>,
    ) -> Vec<CellPosition> {
        let mut available_cells = Vec::new();
        let (allies, enemies) = self.split_pieces_by_color(pieces);

        let is_first_move = (self.color == ChessColor::WHITE
            && self.pos.j == board.first_element + 1)
            || (self.color == ChessColor::BLACK && self.pos.j == board.last_element - 1);
        let direction_cooficient: i8 = if self.color == ChessColor::WHITE {
            1
        } else {
            -1
        };
        let cell_1 = CellPosition {
            i: self.pos.i,
            j: self.pos.j + 1 * direction_cooficient,
        };
        if !allies.contains_key(&cell_1) && !enemies.contains_key(&cell_1) {
            available_cells.push(cell_1);
        }
        let cell_2 = CellPosition {
            i: self.pos.i,
            j: self.pos.j + 2 * direction_cooficient,
        };
        if is_first_move && !allies.contains_key(&cell_2) && !enemies.contains_key(&cell_2) {
            available_cells.push(cell_2);
        }
        let cell_enemy_right = CellPosition {
            i: self.pos.i + 1,
            j: self.pos.j + 1 * direction_cooficient,
        };
        let cell_enemy_left = CellPosition {
            i: self.pos.i - 1,
            j: self.pos.j + 1 * direction_cooficient,
        };
        if enemies.contains_key(&cell_enemy_right) {
            available_cells.push(cell_enemy_right);
        }
        if enemies.contains_key(&cell_enemy_left) {
            available_cells.push(cell_enemy_left);
        }
        return available_cells;
    }

    fn split_pieces_by_color<'a>(
        &self,
        pieces: &'a Vec<&ChessPiece>,
    ) -> (
        HashMap<CellPosition, &'a ChessPiece>,
        HashMap<CellPosition, &'a ChessPiece>,
    ) {
        let mut allies: HashMap<CellPosition, &ChessPiece> = HashMap::new();
        let mut enemies: HashMap<CellPosition, &ChessPiece> = HashMap::new();
        for chess_piece in pieces.iter() {
            if self.color == chess_piece.color {
                allies.insert(chess_piece.pos, *chess_piece);
            } else {
                enemies.insert(chess_piece.pos, *chess_piece);
            }
        }
        return (allies, enemies);
    }
}
