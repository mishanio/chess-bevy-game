use bevy::prelude::*;

use crate::{models::app_state::AppState, titles::Titles};

pub struct UiMenuPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct NewGameButton;

impl Plugin for UiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_ui_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(despawn_ui_menu))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(handle_ui_buttons_styles)
                    .with_system(handle_new_game_button_clicked),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_ui_menu(mut commands: Commands, assets: Res<AssetServer>, titles: Res<Titles>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
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
        })
        .insert(NewGameButton);
}

fn despawn_ui_menu(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for id in button_query.iter() {
        commands.entity(id).despawn_recursive();
    }
}

fn handle_ui_buttons_styles(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn handle_new_game_button_clicked(
    new_game_interaction_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in new_game_interaction_query.iter() {
        if Interaction::Clicked.eq(interaction) {
            app_state.set(AppState::Game).unwrap();
        }
    }
}
