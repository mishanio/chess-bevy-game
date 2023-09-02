use bevy::{app::AppExit, prelude::*};

use crate::{
    models::{
        app_state::AppState,
        common_resources::{FontHolder, GameState},
    },
    titles::Titles,
};

pub struct UiMenuPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
enum MenuButton {
    NewGame,
    Continue,
    Exit,
}

#[derive(Component)]
struct OnGameScreen;

impl Plugin for UiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_ui_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_ui_menu)
            .add_systems(
                Update,
                (handle_ui_buttons_styles, handle_button_clicked)
                    .run_if(in_state(AppState::MainMenu)),
            );

    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_ui_menu(mut commands: Commands, font_holder: Res<FontHolder>, titles: Res<Titles>) {
    let button = ButtonBundle {
        style: Style {
            // size: Size::new(Val::Px(200.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };
    let text_style = TextStyle {
        font: font_holder.font.clone(),
        font_size: 30.,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(OnGameScreen)
        .with_children(|node| {
            node.spawn(button.clone())
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        titles.button_new_game.clone(),
                        text_style.clone(),
                    ));
                })
                .insert(MenuButton::NewGame);
        })
        .with_children(|node| {
            node.spawn(button.clone())
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        titles.button_continue_game.clone(),
                        text_style.clone(),
                    ));
                })
                .insert(MenuButton::Continue);
        })
        .with_children(|node| {
            node.spawn(button.clone())
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        titles.button_exit_game.clone(),
                        text_style.clone(),
                    ));
                })
                .insert(MenuButton::Exit);
        });
}

fn despawn_ui_menu(mut commands: Commands, q_dispawn: Query<Entity, With<OnGameScreen>>) {
    for id in q_dispawn.iter() {
        commands.entity(id).despawn_recursive();
    }
}

fn handle_ui_buttons_styles(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
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

fn handle_button_clicked(
    new_game_interaction_query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, menu_button) in new_game_interaction_query.iter() {
        if Interaction::Pressed.eq(interaction) {
            match menu_button {
                MenuButton::NewGame => {
                    *game_state = GameState::NEW;
                    app_state.set(AppState::Game)
                }
                MenuButton::Continue => {
                    *game_state = GameState::CONTINUE;
                    app_state.set(AppState::Game)
                }
                MenuButton::Exit => exit.send(AppExit),
            }
        }
    }
}
