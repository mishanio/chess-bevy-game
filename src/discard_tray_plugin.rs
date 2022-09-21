use std::collections::HashMap;
use bevy::prelude::*;
// use bevy::utils::HashMap;

use crate::{App, Board, Plugin};
use crate::assets_helper::AssetsHelper;
use crate::models::chess_models::{ChessColor, ChessPieceRemovedEvent, RemovedChessPiece};


#[derive(Default)]
struct DiscardTrayHolder {
    value: HashMap<ChessColor, i8>,
}

pub struct DiscardTrayPlugin;

impl Plugin for DiscardTrayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, set_up_resources)
            .add_system(add_taken_piece_to_discard_tray);
    }
}

fn set_up_resources(mut commands: Commands) {
    commands.insert_resource(DiscardTrayHolder::default())
}


fn add_taken_piece_to_discard_tray(
    mut commands: Commands,
    mut piece_taken_event_reader: EventReader<ChessPieceRemovedEvent>,
    mut discard_tray: ResMut<DiscardTrayHolder>,
    board: Res<Board>,
    assets: Res<AssetServer>,
) {
    piece_taken_event_reader.iter().for_each(|event| {
        let chess_piece = &event.chess_piece;
        let element_num = discard_tray.value.get(&chess_piece.color).map_or(0, |v| *v);

        let removed_piece = RemovedChessPiece {
            color: chess_piece.color.clone(),
            piece_type: chess_piece.piece_type.clone(),
            num: element_num
        };
        AssetsHelper::spawn_removed_piece(removed_piece, &mut commands, &assets, &board);
        discard_tray.value.insert(chess_piece.color.clone(), element_num + 1);

    });
}