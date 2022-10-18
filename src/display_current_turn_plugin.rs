use bevy::{prelude::*, text::Text2dSize};

use crate::{
    assets_helper::AssetsHelper,
    models::{
        app_state::AppState, chess_move_state::MoveState, chess_piece::PieceType,
        common_chess::ChessColor, common_resources::Board,
    },
    titles::Titles,
};

struct TurnImageHolder {
    white_turn_img: Handle<Image>,
    black_turn_img: Handle<Image>,
}

impl TurnImageHolder {
    fn get_image(&self, color: &ChessColor) -> Handle<Image> {
        match color {
            ChessColor::WHITE => self.white_turn_img.clone(),
            ChessColor::BLACK => self.black_turn_img.clone(),
        }
    }
}

#[derive(Component)]
struct CurentTurnImage;

#[derive(Component)]
struct CurentTurnText;

#[derive(Component)]
struct CheckStateText;
#[derive(Component)]
struct MateStateText;

#[derive(Component)]
struct Despawnable;

pub struct DisplayCurrentTurnPlugin;

impl Plugin for DisplayCurrentTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            set_up_display_turn_resource_system,
        )
        // .add_startup_system_to_stage(StartupStage::Startup, set_up_display_turn_components)
        .add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(set_up_display_turn_components),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(display_current_turn_system)
                .with_system(display_check_state_system)
                .with_system(display_mate_state_system),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Game).with_system(despawn_display_turn_components),
        );
    }
}

fn set_up_display_turn_resource_system(mut commands: Commands, assets: Res<AssetServer>) {
    let move_image_holder = TurnImageHolder {
        white_turn_img: AssetsHelper::load_piece_image(
            &ChessColor::WHITE,
            &PieceType::KING,
            &assets,
        ),
        black_turn_img: AssetsHelper::load_piece_image(
            &ChessColor::BLACK,
            &PieceType::KING,
            &assets,
        ),
    };
    commands.insert_resource(move_image_holder);
}

fn set_up_display_turn_components(
    mut commands: Commands,
    assets: Res<AssetServer>,
    board: Res<Board>,
    titles: Res<Titles>,
) {
    let text_x = board.end_x_point();
    let text_y = board.end_y_point() - board.image_size_scaled() / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(board.discard_image_scale()),
                ..default()
            },
            ..default()
        })
        .insert(CurentTurnImage)
        .insert(Despawnable);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                titles.turn.clone(),
                TextStyle {
                    font: assets.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform {
                translation: Vec3::new(text_x, text_y, 0.0),
                scale: Vec3::splat(1.0),
                ..default()
            },
            ..default()
        })
        .insert(CurentTurnText)
        .insert(Despawnable);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                titles.check.clone(),
                TextStyle {
                    font: assets.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::RED,
                },
            ),
            transform: Transform {
                translation: Vec3::new(text_x, text_y - 40., 0.0),
                scale: Vec3::splat(1.0),
                ..default()
            },
            ..default()
        })
        .insert(CheckStateText)
        .insert(Despawnable);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                titles.mate.clone(),
                TextStyle {
                    font: assets.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::RED,
                },
            ),
            transform: Transform {
                translation: Vec3::new(text_x, text_y - 80., 0.0),
                scale: Vec3::splat(1.0),
                ..default()
            },
            ..default()
        })
        .insert(MateStateText)
        .insert(Despawnable);
}

fn despawn_display_turn_components(
    mut commands: Commands,
    q_despawn: Query<Entity, With<Despawnable>>,
) {
    for entity in q_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn display_current_turn_system(
    mut q_current_text: Query<
        (&Transform, &Text2dSize),
        (With<CurentTurnText>, Without<CurentTurnImage>),
    >,
    mut q_current_image: Query<
        (&mut Handle<Image>, &mut Transform),
        (With<CurentTurnImage>, Without<CurentTurnText>),
    >,
    move_state: Res<MoveState>,
    turn_image_holder: Res<TurnImageHolder>,
) {
    let (text_transform, text_size) = q_current_text.single_mut();
    // text.sections[0].value = format!("Current move: {}", color_label);

    let (mut image, mut image_transform) = q_current_image.single_mut();
    *image = turn_image_holder.get_image(&move_state.current_collor);
    image_transform.translation.x = text_transform.translation.x + text_size.size.x + 30.;
    image_transform.translation.y = text_transform.translation.y - text_size.size.y / 2.5;
}

fn display_check_state_system(
    mut q_check_status: Query<&mut Visibility, With<CheckStateText>>,
    move_state: Res<MoveState>,
) {
    let mut check_state_visibility = q_check_status.single_mut();
    check_state_visibility.is_visible = move_state.check_state.is_some();
}

fn display_mate_state_system(
    mut q_mate_status: Query<&mut Visibility, With<MateStateText>>,
    move_state: Res<MoveState>,
) {
    let mut mate_state_visibility = q_mate_status.single_mut();
    mate_state_visibility.is_visible = move_state.mate_state.is_some();
}
