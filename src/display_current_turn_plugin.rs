use bevy::{prelude::*, text::Text2dSize, transform};

use crate::{
    assets_helper::AssetsHelper,
    models::{
        chess_move_state::MoveState, chess_piece::PieceType, common_chess::ChessColor,
        common_resources::Board,
    },
};

pub struct DisplayCurrentTurnPlugin;

impl Plugin for DisplayCurrentTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            set_up_display_turn_resource_system,
        )
        .add_startup_system_to_stage(StartupStage::Startup, set_up_display_turn_components)
        .add_system(display_current_turn_system);
    }
}

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
    turn_image_holder: Res<TurnImageHolder>,
    move_state: Res<MoveState>,
    assets: Res<AssetServer>,
    board: Res<Board>,
) {
    let text_x = board.end_x_point();
    let text_y = board.end_y_point() - board.image_size_scaled() / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            texture: turn_image_holder.get_image(&move_state.current_collor),
            transform: Transform {
                translation: Vec3::new(0., text_y, 1.0),
                scale: Vec3::splat(board.discard_image_scale()),
                ..default()
            },
            ..default()
        })
        .insert(CurentTurnImage);

    let text_style = TextStyle {
        font: assets.load("fonts/FiraMono-Medium.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    };

    let current_move_text_bundle = Text2dBundle {
        text: Text::from_section("Move:", text_style),
        transform: Transform {
            translation: Vec3::new(text_x, text_y, 0.0),
            scale: Vec3::splat(1.0),
            ..default()
        },
        ..default()
    };

    commands
        .spawn_bundle(current_move_text_bundle)
        .insert(CurentTurnText);
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
