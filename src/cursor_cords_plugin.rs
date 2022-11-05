use bevy::{prelude::*, render::camera::RenderTarget};

use crate::models::common_resources::{Board, BoardPointer, MainCamera, FontHolder};

#[derive(Component)]
struct CursorText;

pub struct CursorCordsPlugin;

impl Plugin for CursorCordsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::Startup, set_up_cursor_cords)
            .add_system(set_board_pointer_system)
            .add_system(text_update_system);
    }
}

fn set_up_cursor_cords(mut commands: Commands, font_holder: Res<FontHolder>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "",
                TextStyle {
                    font: font_holder.font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(CursorText);
}

fn set_board_pointer_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    board: Res<Board>,
    mut board_pointer: ResMut<BoardPointer>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        board_pointer.x = world_pos.x + board.board_offset();
        board_pointer.y = world_pos.y + board.board_offset();
        // eprintln!("World coords: {}/{}", board_pointer.x, board_pointer.y);
    }
}

fn text_update_system(bp: Res<BoardPointer>, mut query: Query<&mut Text, With<CursorText>>) {
    for mut text in &mut query {
        text.sections[0].value = format!("World coords: x: {}, y: {}", bp.x, bp.y);
    }
}


