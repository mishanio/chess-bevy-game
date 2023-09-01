use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::{
    assets_helper::AssetsHelper,
    models::{
        app_state::AppState,
        chess_move_state::MoveState,
        chess_piece::PieceType,
        common_chess::ChessColor,
        common_resources::{Board, FontHolder},
    },
    titles::Titles,
};

#[derive(Resource)]
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
        app.add_startup_system(
            set_up_display_turn_resource_system.in_base_set(StartupSet::PreStartup),
        )
        // .add_startup_system_to_stage(StartupStage::Startup, set_up_display_turn_components)
        .add_system(set_up_display_turn_components.in_schedule(OnEnter(AppState::Game)))
        .add_system(despawn_display_turn_components.in_schedule(OnExit(AppState::Game)))
        .add_systems(
            (
                display_current_turn_system,
                display_check_state_system,
                display_mate_state_system,
            )
                .in_set(OnUpdate(AppState::Game)),
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
    font_holder: Res<FontHolder>,
    board: Res<Board>,
    titles: Res<Titles>,
) {
    let text_x = board.end_x_point() + 2. * board.image_size_scaled();
    let text_y = board.end_y_point() - board.image_size_scaled();
    let text_z = 2.0;
    let font_size = board.image_size_scaled() / 2.;

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(board.discard_image_scale()),
                ..default()
            },
            ..default()
        })
        .insert(CurentTurnImage)
        .insert(Despawnable);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                titles.turn.clone(),
                TextStyle {
                    font: font_holder.font.clone(),
                    font_size,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform {
                translation: Vec3::new(text_x, text_y, text_z),
                scale: Vec3::splat(1.0),
                ..default()
            },
            ..default()
        })
        .insert(CurentTurnText)
        .insert(Despawnable);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                titles.check.clone(),
                TextStyle {
                    font: font_holder.font.clone(),
                    font_size: font_size,
                    color: Color::RED,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform {
                translation: Vec3::new(text_x, text_y - font_size, text_z),
                scale: Vec3::splat(1.0),
                ..default()
            },
            ..default()
        })
        .insert(CheckStateText)
        .insert(Despawnable);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                titles.mate.clone(),
                TextStyle {
                    font: font_holder.font.clone(),
                    font_size: font_size,
                    color: Color::RED,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform {
                translation: Vec3::new(text_x, text_y - font_size, text_z),
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
        (&Transform, &Text2dBounds),
        (With<CurentTurnText>, Without<CurentTurnImage>),
    >,
    mut q_current_image: Query<
        (&mut Handle<Image>, &mut Transform),
        (With<CurentTurnImage>, Without<CurentTurnText>),
    >,
    move_state: Res<MoveState>,
    turn_image_holder: Res<TurnImageHolder>,
    board: Res<Board>,
) {

    let (mut image, mut image_transform) = q_current_image.single_mut();

    let (text_transform, text_size) = q_current_text.single_mut();
    // text.sections[0].value = format!("Current move: {}", color_label);

    // let offset_text_image = text_size.size.x;
    let offset_text_image = 40.;
    *image = turn_image_holder.get_image(&move_state.current_collor);
    image_transform.translation.x =
        text_transform.translation.x + offset_text_image + board.image_size_scaled() / 4.;
    image_transform.translation.y = text_transform.translation.y;
}

fn display_check_state_system(
    mut q_check_status: Query<&mut Visibility, With<CheckStateText>>,
    move_state: Res<MoveState>,
) {
    let mut check_state_visibility = q_check_status.single_mut();
    let visibility = match move_state.check_state {
        None => Visibility::Hidden,
        Some(_) => Visibility::Visible,
    };
    *check_state_visibility = visibility;
}

fn display_mate_state_system(
    mut q_mate_status: Query<&mut Visibility, With<MateStateText>>,
    move_state: Res<MoveState>,
) {
    let mut mate_state_visibility = q_mate_status.single_mut();
    let visibility = match move_state.check_state {
        None => Visibility::Hidden,
        Some(_) => Visibility::Visible,
    };
    *mate_state_visibility = visibility;
}
