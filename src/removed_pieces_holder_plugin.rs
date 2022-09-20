use bevy::prelude::*;
use crate::{App, Plugin};
use crate::models::chess_models::{ChessPiece, ChessPieceRemovedEvent};



#[derive(Default)]
struct RemovedPieceStorage {
    last_white_position: f32,
    last_black_position: f32,
}

pub struct RemovedPiecesHolderPlugin;

impl Plugin for RemovedPiecesHolderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, set_up_resources)
            .add_system(remove_taken_piece_system);
    }
}

fn set_up_resources(mut commands: Commands) {
    commands.insert_resource(RemovedPieceStorage::default())
}

fn remove_taken_piece_system(
    mut commands: Commands,
    q_chess_piece: Query<(Entity, &ChessPiece)>,
    mut piece_taken_event_reader: EventReader<ChessPieceRemovedEvent>,
) {
    piece_taken_event_reader.iter().for_each(|event| {
        q_chess_piece.iter().for_each(|(entity, cp)| {
            if cp.pos == event.pos && cp.color == event.color {
                commands.entity(entity).despawn();
            }
        })
    })
}