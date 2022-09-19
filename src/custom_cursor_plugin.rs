use bevy::prelude::*;

use crate::models::common_resources::BoardPointer;


const CURSOR_SCALE: f32 = 0.7;


#[derive(Component)]
struct CustomCursor;

pub struct CustomCursorPlugin;

impl Plugin for CustomCursorPlugin {

    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, hide_default_cursor)
        .add_startup_system_to_stage(StartupStage::Startup, insert_custom_cursor)
        .add_system(move_cursor);
    }
}

fn hide_default_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
}

fn insert_custom_cursor(mut commands: Commands, assets: Res<AssetServer>) {
    let custom_cursor = assets.load("cursor/point-64.png");

    commands
    .spawn_bundle(SpriteBundle {
        texture: custom_cursor,
        transform: Transform {
            translation: Vec3::new(0., 0., 2.0),
            scale: Vec3::splat(CURSOR_SCALE),
            ..default()
        },
        ..Default::default()
    })
    .insert(CustomCursor);
}

fn move_cursor(board_pionter: Res<BoardPointer>, mut q_cursor: Query<&mut Transform, With<CustomCursor>>) {
    let mut custom_cursor_transform = q_cursor.single_mut();
    custom_cursor_transform.translation.x = board_pionter.x - 34. * CURSOR_SCALE;
    custom_cursor_transform.translation.y = board_pionter.y - 64. * CURSOR_SCALE;
    
}