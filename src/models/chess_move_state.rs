use bevy::prelude::*;

use super::common_chess::ChessColor;

#[derive(Default)]
pub struct MoveState {
    pub selected_piece: Option<Entity>,
    pub selected_cell: Option<Entity>,
    pub move_in_action: bool,
    pub current_collor: ChessColor,

    pub check_state: Option<ChessColor>,
    pub mate_state: Option<ChessColor>,
    pub is_stalemate_state: Option<ChessColor>,
}

impl MoveState {
    pub fn next_move(&mut self) {
        self.move_in_action = false;
        self.selected_cell = None;
        self.selected_piece = None;
        self.current_collor = self.current_collor.opposite();
    }
}
