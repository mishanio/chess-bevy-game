use bevy::prelude::*;

use chess_board_plugin::ChessBoardPlugin;
use cursor_cords_plugin::CursorCordsPlugin;
use custom_cursor_plugin::CustomCursorPlugin;
use display_current_turn_plugin::DisplayCurrentTurnPlugin;
use models::{
    app_state::AppState,
    common_resources::{Board, BoardPointer, FontHolder, GameState, MainCamera},
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
    let window_title = titles.title.clone();
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.30, 0.40)))
        .insert_resource(GameState::NEW)
        .insert_resource(titles)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: window_title,
                // width: 1920.,
                // height: 1080.,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_startup_systems(
            (set_up_resources, set_up_font_resource).in_base_set(StartupSet::PreStartup),
        )
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
    commands.insert_resource(Board::new(-220., -220., 128., 0.48));

    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn set_up_font_resource(mut commands: Commands, assets: Res<AssetServer>) {
    warn!("main insert FontHolder");
    commands.insert_resource(FontHolder {
        font: assets.load("fonts/FiraMono-Medium.ttf"),
    });
}

fn change_game_state(
    mut keys: ResMut<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    // match app_state.current() {
    //     AppState::MainMenu => app_state.set(AppState::Game).unwrap(),
    //     AppState::Game => app_state.set(AppState::MainMenu).unwrap(),
    // };

    if let AppState::Game = app_state.0 {
        next_state.set(AppState::MainMenu);
    }
    keys.reset(KeyCode::Escape);
}
