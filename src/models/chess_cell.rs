use bevy::prelude::*;

use super::{common_resources::CellPosition, common_chess::ChessColor};

pub enum ChessCellState {
    NONE,
    HIGHLIGHTED,
    SELECTED,
    ATTACKED,
}

#[derive(Component)]
pub struct ChessCell {
    pub pos: CellPosition,
    pub state: ChessCellState,
}
impl ChessCell {
    pub fn from(i: i8, j: i8) -> ChessCell {
        ChessCell {
            pos: CellPosition { i, j },
            state: ChessCellState::NONE,
        }
    }
    pub fn color(&self) -> ChessColor {
        return if (self.pos.j + self.pos.i) % 2 == 0 {
            ChessColor::WHITE
        } else {
            ChessColor::BLACK
        };
    }
}