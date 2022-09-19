use std::cell::Cell;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

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
        let (ally_cells, enemy_cells) = self.split_pieces_by_color(pieces);

        return match self.piece_type {
            PieceType::PAWN => {
                return self.available_cells_for_pawn(board, &ally_cells, &enemy_cells);
            }
            PieceType::ROOK => {
                return self.available_cells_for_rook(board, &ally_cells, &enemy_cells)
            }
            _ => vec![],
        };
    }

    fn available_cells_for_pawn(
        &self,
        board: &Board,
        ally_cells: &HashSet<CellPosition>,
        enemy_cells: &HashSet<CellPosition>,
    ) -> Vec<CellPosition> {
        let mut available_cells = Vec::new();
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
        if !ally_cells.contains(&cell_1) && !enemy_cells.contains(&cell_1) {
            available_cells.push(cell_1);
        }
        let cell_2 = CellPosition {
            i: self.pos.i,
            j: self.pos.j + 2 * direction_cooficient,
        };
        if is_first_move && !ally_cells.contains(&cell_2) && !enemy_cells.contains(&cell_2) {
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
        if enemy_cells.contains(&cell_enemy_right) {
            available_cells.push(cell_enemy_right);
        }
        if enemy_cells.contains(&cell_enemy_left) {
            available_cells.push(cell_enemy_left);
        }
        return available_cells;
    }

    fn available_cells_for_rook(
        &self,
        board: &Board,
        ally_cells: &HashSet<CellPosition>,
        enemy_cells: &HashSet<CellPosition>,
    ) -> Vec<CellPosition> {
        let range_right: Vec<CellPosition> = (self.pos.i + 1..board.last_element + 1)
            .map(|i| CellPosition { i, j: self.pos.j })
            .collect();
        let range_left: Vec<CellPosition> = (board.first_element..self.pos.i)
            .rev()
            .map(|i| CellPosition { i, j: self.pos.j })
            .collect();
        let range_down: Vec<CellPosition> = (board.first_element..self.pos.j)
            .rev()
            .map(|j| CellPosition { i: self.pos.i, j })
            .collect();
        let range_up: Vec<CellPosition> = (self.pos.j + 1..board.last_element + 1)
            .map(|j| CellPosition { i: self.pos.i, j })
            .collect();

        let available_line_cells = |range| {
            let mut available_cells = Vec::new();
            for cell in range {
                if ally_cells.contains(&cell) {
                    break;
                }
                if enemy_cells.contains(&cell) {
                    available_cells.push(cell);
                    break;
                }
                available_cells.push(cell)
            }
            return available_cells;
        };

        let range_right = available_line_cells(range_right);
        let range_left = available_line_cells(range_left);
        let range_up = available_line_cells(range_up);
        let range_down = available_line_cells(range_down);

        return [range_right, range_left, range_down, range_up].concat();
    }

    fn split_pieces_by_color(
        &self,
        pieces: &Vec<&ChessPiece>,
    ) -> (HashSet<CellPosition>, HashSet<CellPosition>) {
        let mut allies = HashSet::new();
        let mut enemies = HashSet::new();
        for chess_piece in pieces.iter() {
            if self.color == chess_piece.color {
                allies.insert(chess_piece.pos);
            } else {
                enemies.insert(chess_piece.pos);
            }
        }
        return (allies, enemies);
    }

    // fn split_pieces_by_color_map<'a>(
    //     &self,
    //     pieces: &'a Vec<&ChessPiece>,
    // ) -> (
    //     HashMap<CellPosition, &'a ChessPiece>,
    //     HashMap<CellPosition, &'a ChessPiece>,
    // ) {
    //     let mut allies: HashMap<CellPosition, &ChessPiece> = HashMap::new();
    //     let mut enemies: HashMap<CellPosition, &ChessPiece> = HashMap::new();
    //     for chess_piece in pieces.iter() {
    //         if self.color == chess_piece.color {
    //             allies.insert(chess_piece.pos, *chess_piece);
    //         } else {
    //             enemies.insert(chess_piece.pos, *chess_piece);
    //         }
    //     }
    //     return (allies, enemies);
    // }
}
