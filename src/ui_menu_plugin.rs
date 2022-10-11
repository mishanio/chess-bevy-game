use bevy::prelude::*;

use crate::{models::app_state::AppState, titles::Titles};

pub struct UiMenuPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl Plugin for UiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_ui_menu));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_ui_menu(mut commands: Commands, assets: Res<AssetServer>, titles: Res<Titles>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|button| {
            button.spawn_bundle(TextBundle::from_section(
                titles.button_new_game.clone(),
                TextStyle {
                    font: assets.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
