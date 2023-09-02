use bevy::{prelude::*, render::camera::RenderTarget, window::PrimaryWindow};

use crate::models::common_resources::{Board, BoardPointer, FontHolder, MainCamera};

#[derive(Component)]
struct CursorText;

pub struct CursorCordsPlugin;

impl Plugin for CursorCordsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_up_cursor_cords)
        .add_systems(Update, set_board_pointer_system);
        // .add_system(text_update_system);
    }
}

fn set_up_cursor_cords(mut commands: Commands, font_holder: Res<FontHolder>) {
    warn!("CursorCordsPlugin get FontHolder");
    commands
        .spawn(
            // Create a TextBundle that has a Text with a single sections.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "",
                TextStyle {
                    font: font_holder.font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                // position: UiRect {
                //     bottom: Val::Px(5.0),
                //     right: Val::Px(15.0),
                //     ..default()
                // },
                ..default()
            }),
        )
        .insert(CursorText);
}

fn set_board_pointer_system(
    // need to get window dimensions
    primary_window: Query<(&Window, &PrimaryWindow)>,
    all_windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    board: Res<Board>,
    mut board_pointer: ResMut<BoardPointer>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)

    let window: &Window = if let RenderTarget::Window(window_ref) = camera.target {
        match window_ref {
            bevy::window::WindowRef::Entity(id) => all_windows.get(id).unwrap(),
            bevy::window::WindowRef::Primary => primary_window.single().0,
        }
    } else {
        primary_window.single().0
    };

    //  check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        board_pointer.x = world_position.x + board.board_offset();
        board_pointer.y = world_position.y + board.board_offset();
    }
}

fn text_update_system(bp: Res<BoardPointer>, mut query: Query<&mut Text, With<CursorText>>) {
    for mut text in &mut query {
        text.sections[0].value = format!("World coords: x: {}, y: {}", bp.x, bp.y);
    }
}
