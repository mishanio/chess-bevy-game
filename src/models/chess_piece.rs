use bevy::{prelude::*, utils::HashSet};

use crate::models::common_resources::{Board, CellPosition};

use super::common_chess::ChessColor;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum PieceType {
    PAWN,
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    KING,
}

struct DiagonalCellIter {
    pos: CellPosition,
    first_element: i8,
    last_element: i8,
    x_direction: i8,
    y_direction: i8,
}

#[derive(Component, Clone, Debug)]
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
            PieceType::PAWN => self.available_cells_for_pawn(board, &ally_cells, &enemy_cells),
            PieceType::ROOK => self.available_cells_for_rook(board, &ally_cells, &enemy_cells),
            PieceType::BISHOP => self.available_cells_for_bishop(board, &ally_cells, &enemy_cells),
            PieceType::KNIGHT => self.available_cells_for_knight(board, &ally_cells, &enemy_cells),
            PieceType::QUEEN => self.available_cells_for_queen(board, &ally_cells, &enemy_cells),
            PieceType::KING => self.available_cells_for_king(board, pieces, false),
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
        let direction_coefficient: i8 = match self.color {
            ChessColor::WHITE => 1,
            ChessColor::BLACK => -1,
        };

        let cell_1 = CellPosition {
            i: self.pos.i,
            j: self.pos.j + 1 * direction_coefficient,
        };
        if !ally_cells.contains(&cell_1) && !enemy_cells.contains(&cell_1) {
            available_cells.push(cell_1);
        }
        let cell_2 = CellPosition {
            i: self.pos.i,
            j: self.pos.j + 2 * direction_coefficient,
        };
        if is_first_move && !ally_cells.contains(&cell_2) && !enemy_cells.contains(&cell_2) {
            available_cells.push(cell_2);
        }
        let cell_enemy_right = CellPosition {
            i: self.pos.i + 1,
            j: self.pos.j + 1 * direction_coefficient,
        };
        let cell_enemy_left = CellPosition {
            i: self.pos.i - 1,
            j: self.pos.j + 1 * direction_coefficient,
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

        let range_right = self.available_line_cells(range_right, ally_cells, enemy_cells);
        let range_left = self.available_line_cells(range_left, ally_cells, enemy_cells);
        let range_up = self.available_line_cells(range_up, ally_cells, enemy_cells);
        let range_down = self.available_line_cells(range_down, ally_cells, enemy_cells);

        return [range_right, range_left, range_down, range_up].concat();
    }

    fn available_cells_for_bishop(
        &self,
        board: &Board,
        ally_cells: &HashSet<CellPosition>,
        enemy_cells: &HashSet<CellPosition>,
    ) -> Vec<CellPosition> {
        let range_top_right: Vec<CellPosition> =
            DiagonalCellIter::top_right(self.pos, board).collect();
        let range_top_left: Vec<CellPosition> =
            DiagonalCellIter::top_left(self.pos, board).collect();
        let range_down_right: Vec<CellPosition> =
            DiagonalCellIter::down_right(self.pos, board).collect();
        let range_down_left: Vec<CellPosition> =
            DiagonalCellIter::down_left(self.pos, board).collect();

        let range_top_right = self.available_line_cells(range_top_right, ally_cells, enemy_cells);
        let range_top_left = self.available_line_cells(range_top_left, ally_cells, enemy_cells);
        let range_down_right = self.available_line_cells(range_down_right, ally_cells, enemy_cells);
        let range_down_left = self.available_line_cells(range_down_left, ally_cells, enemy_cells);

        return [
            range_top_right,
            range_top_left,
            range_down_right,
            range_down_left,
        ]
        .concat();
    }

    fn available_cells_for_knight(
        &self,
        board: &Board,
        ally_cells: &HashSet<CellPosition>,
        _enemy_cells: &HashSet<CellPosition>,
    ) -> Vec<CellPosition> {
        return vec![
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ]
        .iter()
        .map(|(i, j)| CellPosition {
            i: self.pos.i + *i as i8,
            j: self.pos.j + *j as i8,
        })
        .filter(|cell_position| {
            !board.is_cell_out_of_range(cell_position) && !ally_cells.contains(cell_position)
        })
        .collect();
    }

    fn available_cells_for_queen(
        &self,
        board: &Board,
        ally_cells: &HashSet<CellPosition>,
        enemy_cells: &HashSet<CellPosition>,
    ) -> Vec<CellPosition> {
        let cells_for_rook = self.available_cells_for_rook(board, ally_cells, enemy_cells);
        let cells_for_bishop = self.available_cells_for_bishop(board, ally_cells, enemy_cells);
        return [cells_for_rook, cells_for_bishop].concat();
    }

    fn available_cells_for_king(
        &self,
        board: &Board,
        pieces: &Vec<&ChessPiece>,
        skip_check_enemy_king_state: bool,
    ) -> Vec<CellPosition> {
        let (ally_cells, enemy_cells) = self.split_pieces_by_color(pieces);

        let available_by_distance = |cell_position: &CellPosition| -> bool {
            let is_out_of_range = cell_position.i > self.pos.i + 1
                || cell_position.i < self.pos.i - 1
                || cell_position.j > self.pos.j + 1
                || cell_position.j < self.pos.j - 1;
            return !is_out_of_range;
        };

        let not_on_enemy_path = |cell_position: &CellPosition| -> bool {
            if skip_check_enemy_king_state {
                return true;
            }
            !ChessPiece::is_cell_on_enemy_path(&self.color, cell_position, pieces, board)
        };

        let cells: Vec<CellPosition> = self
            .available_cells_for_queen(board, &ally_cells, &enemy_cells)
            .iter()
            .map(|cp| *cp)
            .filter(available_by_distance)
            .filter(not_on_enemy_path)
            .collect();

        return cells;
    }

    fn is_cell_on_enemy_path(
        color: &ChessColor,
        cell_position: &CellPosition,
        pieces: &Vec<&ChessPiece>,
        board: &Board,
    ) -> bool {
        let enemy_pieces: Vec<&ChessPiece> = pieces
            .iter()
            .map(|cp| *cp)
            .filter(|chess_piece| !chess_piece.color.eq(color))
            .collect();
        for enemy_piece in &enemy_pieces {
            if enemy_piece.piece_type == PieceType::KING {
                if enemy_piece
                    .available_cells_for_king(board, pieces, true)
                    .contains(cell_position)
                {
                    return true;
                }
                continue;
            }

            for enemy_available_cel_position in
                enemy_piece.get_available_cells_for_move(board, pieces)
            {
                if enemy_available_cel_position.eq(cell_position) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn pieces_after_move<'a>(
        pieces: &Vec<&'a ChessPiece>,
        cell_position: &CellPosition,
        cloned_selected_piece: &'a mut ChessPiece,
    ) -> (Option<&'a ChessPiece>, Vec<&'a ChessPiece>) {
        let maybe_removed_piece = pieces
            .iter()
            .find(|chess_piece| chess_piece.pos.eq(cell_position))
            .map(|cp| *cp);

        let mut pieces_after_move: Vec<&ChessPiece> = pieces
            .iter()
            .filter(|piece| piece.pos != cloned_selected_piece.pos)
            .filter(|piece| {
                maybe_removed_piece
                    .filter(|rm_piece| piece.pos == rm_piece.pos)
                    .is_none()
            })
            .map(|p| *p)
            .collect();

        cloned_selected_piece.pos = cell_position.clone();
        pieces_after_move.push(cloned_selected_piece);
        (maybe_removed_piece, pieces_after_move)
    }

    pub fn is_king_under_check(
        color: &ChessColor,
        pieces: &Vec<&ChessPiece>,
        board: &Board,
    ) -> bool {
        let king = pieces
            .iter()
            .find(|piece| piece.color.eq(color) && piece.piece_type == PieceType::KING);

        if king.is_none() {
            return false;
        }
        let king_position = king.unwrap().pos;

        return ChessPiece::is_cell_on_enemy_path(color, &king_position, pieces, board);
    }

    pub fn is_king_under_mate(
        color: &ChessColor,
        pieces: &Vec<&ChessPiece>,
        board: &Board,
    ) -> bool {
        let ally_pieces: Vec<&ChessPiece> = pieces
            .iter()
            .filter(|piece| piece.color.eq(color))
            .map(|cp| *cp)
            .collect();
        debug!("check ally_pieces {:?}", ally_pieces);
        for ally_piece in ally_pieces {
            debug!("check ally_piece {:?}", ally_piece);
            for cell_position in ally_piece.get_available_cells_for_move(board, pieces) {
                let mut cloned_selected_piece = ally_piece.clone();
                let (_, pieces_after_move) = ChessPiece::pieces_after_move(
                    pieces,
                    &cell_position,
                    &mut cloned_selected_piece,
                );

                if !ChessPiece::is_king_under_check(color, &pieces_after_move, board) {
                    return false;
                }
            }
        }
        return true;
    }

    fn available_line_cells(
        &self,
        range: Vec<CellPosition>,
        ally_cells: &HashSet<CellPosition>,
        enemy_cells: &HashSet<CellPosition>,
    ) -> Vec<CellPosition> {
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

impl DiagonalCellIter {
    fn top_right(pos: CellPosition, board: &Board) -> DiagonalCellIter {
        return DiagonalCellIter {
            pos,
            first_element: board.first_element,
            last_element: board.last_element,
            x_direction: 1,
            y_direction: 1,
        };
    }

    fn top_left(pos: CellPosition, board: &Board) -> DiagonalCellIter {
        return DiagonalCellIter {
            pos,
            first_element: board.first_element,
            last_element: board.last_element,
            x_direction: -1,
            y_direction: 1,
        };
    }

    fn down_right(pos: CellPosition, board: &Board) -> DiagonalCellIter {
        return DiagonalCellIter {
            pos,
            first_element: board.first_element,
            last_element: board.last_element,
            x_direction: 1,
            y_direction: -1,
        };
    }

    fn down_left(pos: CellPosition, board: &Board) -> DiagonalCellIter {
        return DiagonalCellIter {
            pos,
            first_element: board.first_element,
            last_element: board.last_element,
            x_direction: -1,
            y_direction: -1,
        };
    }

    //todo same methods Board has
    fn is_cell_out_of_range(&self, cell: &CellPosition) -> bool {
        return self.is_out_of_range(cell.i) || self.is_out_of_range(cell.j);
    }
    fn is_out_of_range(&self, pos: i8) -> bool {
        pos < self.first_element || pos > self.last_element
    }
}

impl Iterator for DiagonalCellIter {
    type Item = CellPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let next = CellPosition {
            i: self.pos.i + self.x_direction,
            j: self.pos.j + self.y_direction,
        };
        if self.is_cell_out_of_range(&next) {
            return None;
        }
        self.pos = next;
        return Some(self.pos);
    }
}

#[cfg(test)]
mod run_tests {

    use super::*;

    #[test]
    fn test_chess_piece_king_mate_true() {
        let rook1 = ChessPiece::new(0, 7, ChessColor::WHITE, PieceType::ROOK);
        let rook2 = ChessPiece::new(0, 6, ChessColor::WHITE, PieceType::ROOK);
        let king = ChessPiece::new(5, 7, ChessColor::BLACK, PieceType::KING);

        let pieces = vec![&rook1, &rook2, &king];
        let board = Board::new(-200., -200., 128., 0.5);
        let is_mate = ChessPiece::is_king_under_mate(&ChessColor::BLACK, &pieces, &board);
        assert_eq!(true, is_mate);
    }

    #[test]
    fn test_chess_piece_king_mate_false() {
        let rook1 = ChessPiece::new(0, 7, ChessColor::WHITE, PieceType::ROOK);
        let rook2 = ChessPiece::new(0, 6, ChessColor::WHITE, PieceType::ROOK);
        let king = ChessPiece::new(5, 6, ChessColor::BLACK, PieceType::KING);

        let pieces = vec![&rook1, &rook2, &king];
        let board = Board::new(-200., -200., 128., 0.5);
        let is_mate = ChessPiece::is_king_under_mate(&ChessColor::BLACK, &pieces, &board);
        assert_eq!(false, is_mate);
    }
}
