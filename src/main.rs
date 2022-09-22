use bevy::prelude::*;

use chess_board_plugin::ChessBoardPlugin;
use cursor_cords_plugin::CursorCordsPlugin;
use custom_cursor_plugin::CustomCursorPlugin;
use models::common_resources::{Board, BoardPointer, MainCamera};

use crate::discard_tray_plugin::DiscardTrayPlugin;

mod cursor_cords_plugin;
mod chess_board_plugin;
mod custom_cursor_plugin;
mod models;
mod discard_tray_plugin;
mod assets_helper;
mod piece_parser;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.30, 0.40)))
        .insert_resource(WindowDescriptor {
            title: "Chess Game".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, set_up_resources)
        .add_plugin(ChessBoardPlugin)
        .add_plugin(CursorCordsPlugin)
        .add_plugin(CustomCursorPlugin)
        .add_plugin(DiscardTrayPlugin)
        .run();
}

fn set_up_resources(mut commands: Commands) {
    commands.insert_resource(BoardPointer { x: 0., y: 0. });
    commands.insert_resource(Board::new(-200., -200., 128., 0.5));

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}