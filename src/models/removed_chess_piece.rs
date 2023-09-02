use bevy::prelude::{Component, Event};

use super::{
    chess_piece::{ChessPiece, PieceType},
    common_chess::ChessColor,
};

#[derive(Event)]
pub struct ChessPieceRemovedEvent {
    pub chess_piece: ChessPiece,
}

#[derive(Component)]
pub struct RemovedChessPiece {
    pub color: ChessColor,
    pub piece_type: PieceType,
    pub num: i8,
}
