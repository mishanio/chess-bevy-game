use bevy::{prelude::*, window::WindowMode};

use chess_board_plugin::ChessBoardPlugin;
use cursor_cords_plugin::CursorCordsPlugin;
use custom_cursor_plugin::CustomCursorPlugin;
use display_current_turn_plugin::DisplayCurrentTurnPlugin;
use models::{
    app_state::AppState,
    common_resources::{Board, BoardPointer, GameState, MainCamera},
};
use titles::{TitleLocale, Titles};
use ui_menu_plugin::UiMenuPlugin;

use crate::discard_tray_plugin::DiscardTrayPlugin;

mod assets_helper;
mod chess_board_plugin;
mod cursor_cords_plugin;
mod custom_cursor_plugin;
mod discard_tray_plugin;
mod display_current_turn_plugin;
mod models;
mod piece_parser;
mod titles;
mod ui_menu_plugin;

fn main() {
    let titles = Titles::new(TitleLocale::RU);
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.30, 0.40)))
        .insert_resource(WindowDescriptor {
            title: titles.title.clone(),
            // width: 1920.,
            // height: 1080.,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(GameState::NEW)
        .insert_resource(titles)
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        .add_startup_system_to_stage(StartupStage::PreStartup, set_up_resources)
        .add_plugin(ChessBoardPlugin)
        .add_plugin(CursorCordsPlugin)
        .add_plugin(CustomCursorPlugin)
        .add_plugin(DiscardTrayPlugin)
        .add_plugin(DisplayCurrentTurnPlugin)
        .add_plugin(UiMenuPlugin)
        .add_system(change_game_state)
        .run();
}

fn set_up_resources(mut commands: Commands) {
    commands.insert_resource(BoardPointer { x: 0., y: 0. });
    commands.insert_resource(Board::new(-200., -200., 128., 0.5));

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}

fn change_game_state(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    // match app_state.current() {
    //     AppState::MainMenu => app_state.set(AppState::Game).unwrap(),
    //     AppState::Game => app_state.set(AppState::MainMenu).unwrap(),
    // };

    if let AppState::Game = app_state.current() {
        app_state.set(AppState::MainMenu).unwrap();
    }
    keys.reset(KeyCode::Escape);
}
