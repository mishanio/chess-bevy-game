use bevy::prelude::*;
use std::collections::HashMap;

use crate::assets_helper::AssetsHelper;
use crate::models::app_state::AppState;
use crate::models::chess_piece::PieceType;
use crate::models::common_chess::ChessColor;
use crate::models::common_resources::GameState;
use crate::models::removed_chess_piece::{ChessPieceRemovedEvent, RemovedChessPiece};
use crate::{App, Board, Plugin};

#[derive(Default, Resource)]
struct DiscardTrayHolder {
    value: HashMap<ChessColor, i8>,
}
#[derive(Default, Resource)]
struct DiscardPiecesStore {
    state: Vec<(ChessColor, PieceType)>,
}

pub struct DiscardTrayPlugin;

impl Plugin for DiscardTrayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DiscardTrayHolder::default())
            .insert_resource(DiscardPiecesStore::default())
            .add_systems(OnEnter(AppState::Game), set_up_resources)
            .add_systems(OnExit(AppState::Game),despawn_discard_tray_pieces)
            .add_systems(Update, add_taken_piece_to_discard_tray.run_if(in_state(AppState::Game)));
    }
}

fn set_up_resources(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut discard_tray_holder: ResMut<DiscardTrayHolder>,
    mut pieces_store: ResMut<DiscardPiecesStore>,
    board: Res<Board>,
    assets: Res<AssetServer>,
) {
    if let GameState::NEW = *game_state {
        pieces_store.state = vec![];
    }
    for (color, piece_type) in pieces_store.state.iter() {
        add_to_dicard(
            color,
            piece_type,
            &mut discard_tray_holder,
            &mut commands,
            &assets,
            &board,
        );
    }
}

fn despawn_discard_tray_pieces(
    mut commands: Commands,
    q_despawn: Query<(Entity, &RemovedChessPiece)>,
    mut pieces_store: ResMut<DiscardPiecesStore>,
    mut discard_tray_holder: ResMut<DiscardTrayHolder>,
) {
    pieces_store.state = vec![];
    discard_tray_holder.as_mut().value = HashMap::new();
    for (entity, chess_piece) in q_despawn.iter() {
        pieces_store
            .state
            .push((chess_piece.color.clone(), chess_piece.piece_type.clone()));
        commands.entity(entity).despawn();
    }
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
        let piece_type = chess_piece.piece_type.clone();
        let color = chess_piece.color.clone();

        add_to_dicard(
            &color,
            &piece_type,
            &mut discard_tray,
            &mut commands,
            &assets,
            &board,
        );
    });
}

fn add_to_dicard(
    color: &ChessColor,
    piece_type: &PieceType,
    discard_tray: &mut DiscardTrayHolder,
    commands: &mut Commands,
    assets: &AssetServer,
    board: &Board,
) {
    let element_num = discard_tray.value.get(&color).map_or(0, |v| *v);
    let removed_piece = RemovedChessPiece {
        color: color.clone(),
        piece_type: piece_type.clone(),
        num: element_num,
    };
    AssetsHelper::spawn_removed_piece(removed_piece, commands, &assets, &board);
    discard_tray.value.insert(color.clone(), element_num + 1);
}
