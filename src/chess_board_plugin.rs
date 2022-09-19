use std::borrow::BorrowMut;

use bevy::prelude::*;

use crate::{
    models::chess_models::{ChessCell, ChessColor, ChessPiece, MoveState, PieceType},
    models::common_resources::{Board, BoardPointer},
};

pub struct ChessBoardPlugin;

impl Plugin for ChessBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, set_up_resources)
            .add_startup_system_to_stage(StartupStage::Startup, set_up_chess_board)
            .add_startup_system_to_stage(StartupStage::Startup, set_up_chess_pieces)
            .add_system(highlight_chess_piece_system)
            .add_system(highlight_chess_cell_system)
            .add_system(set_piece_selected)
            .add_system(set_cell_selected)
            .add_system(move_piece);
    }
}

fn set_up_resources(mut commands: Commands) {
    commands.insert_resource(MoveState::default());
}

fn set_up_chess_board(assets: Res<AssetServer>, mut commands: Commands, board: Res<Board>) {
    let dark_square = assets.load("shadowed/128px/square brown dark_png_shadow_128px.png");
    let light_square = assets.load("shadowed/128px/square brown light_png_shadow_128px.png");

    for j in board.cell_range() {
        for i in board.cell_range() {
            let y = board.y_coordinate(j);
            let x = board.x_coordinate(i);
            let square = if (j + i) % 2 == 0 {
                dark_square.clone()
            } else {
                light_square.clone()
            };

            commands
                .spawn_bundle(SpriteBundle {
                    texture: square.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: Vec3::splat(board.image_scale),
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(ChessCell::from(i, j));
        }
    }
}

fn set_up_chess_pieces(assets: Res<AssetServer>, mut commands: Commands, board: Res<Board>) {
    let first = board.first_element;
    let last = board.last_element;
    spawn_piece(
        ChessPiece::new(first,last, ChessColor::BLACK, PieceType::ROOK),
        commands.borrow_mut(),
        &assets,
        &board,
    );

    spawn_piece(
        ChessPiece::new(first + 3, first + 1, ChessColor::WHITE, PieceType::PAWN),
        commands.borrow_mut(),
        &assets,
        &board,
    );
    spawn_piece(
        ChessPiece::new(first + 4, first + 1, ChessColor::WHITE, PieceType::PAWN),
        commands.borrow_mut(),
        &assets,
        &board,
    );
    spawn_piece(
        ChessPiece::new(first + 4, last - 1, ChessColor::BLACK, PieceType::PAWN),
        commands.borrow_mut(),
        &assets,
        &board,
    );
}

fn highlight_chess_piece_system(
    mut q_chess_cells: Query<(Entity, &mut Transform, &ChessPiece)>,
    board_pointer: Res<BoardPointer>,
    move_sate: Res<MoveState>,
    board: Res<Board>,
) {
    if move_sate.move_in_action {
        return;
    }
    for (entity, mut transform, chess_piece) in q_chess_cells.iter_mut() {
        if Some(entity).eq(&move_sate.selected_piece) {
            transform.scale = Vec3::splat(board.image_scale * 1.1);
        } else if board.is_cell_matches(&chess_piece.pos, &board_pointer) {
            transform.scale = Vec3::splat(board.image_scale * 1.05);
        } else {
            transform.scale = Vec3::splat(board.image_scale * 1.0);
        }
    }
}

fn highlight_chess_cell_system(
    mut q_chess_cells: Query<(Entity, &mut Sprite, &ChessCell)>,
    board_pointer: Res<BoardPointer>,
    move_state: Res<MoveState>,
    q_chess_piece: Query<&ChessPiece>,
    board: Res<Board>,
) {
    if move_state.move_in_action {
        return;
    }
    if move_state.selected_piece.is_none() {
        return;
    }

    let selected_piece = q_chess_piece
        .get(move_state.selected_piece.unwrap())
        .unwrap();

    let pieces: Vec<&ChessPiece> = q_chess_piece.iter().collect();

    let available_pieces = selected_piece.get_available_cells_for_move(&board, &pieces);

    for (entity, mut sprite, chess_cell) in q_chess_cells.iter_mut() {
        let is_current_cell_selected = selected_piece.pos == chess_cell.pos;
        if is_current_cell_selected {
            sprite.color.set_r(1.);
            continue;
        }

        if board.is_cell_matches(&chess_cell.pos, &board_pointer) {
            sprite.color.set_r(0.5);
        } else if available_pieces.contains(&chess_cell.pos) {
            sprite.color.set_r(0.7);
        } else {
            sprite.color.set_r(1.);
        }
    }
}

fn set_piece_selected(
    buttons: Res<Input<MouseButton>>,
    pointer: Res<BoardPointer>,
    board: Res<Board>,
    mut move_sate: ResMut<MoveState>,
    q_chess_piece: Query<(Entity, &ChessPiece)>,
) {
    if move_sate.move_in_action {
        return;
    }
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, piece) in q_chess_piece.iter() {
        if board.is_cell_matches(&piece.pos, &pointer) {
            if Some(entity) == move_sate.selected_piece {
                move_sate.selected_piece = None
            } else {
                move_sate.selected_piece = Some(entity)
            }
        }
    }
}

fn set_cell_selected(
    buttons: Res<Input<MouseButton>>,
    pointer: Res<BoardPointer>,
    board: Res<Board>,
    mut move_state: ResMut<MoveState>,
    q_chess_cell: Query<(Entity, &ChessCell)>,
    q_chess_piece: Query<&ChessPiece>,
) {
    if move_state.move_in_action {
        return;
    }

    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if move_state.selected_piece.is_none() {
        return;
    }

    let selected_piece = q_chess_piece
        .get(move_state.selected_piece.unwrap())
        .unwrap();

    for (entity, cell) in q_chess_cell.iter() {
        let is_current_cell_selected = selected_piece.pos == cell.pos;
        if is_current_cell_selected {
            continue;
        }
        if board.is_cell_matches(&cell.pos, &pointer) {
            debug!("move_in action set to true");
            move_state.selected_cell = Some(entity);
            move_state.move_in_action = true;
        }
    }
}

fn move_piece(
    time: Res<Time>,
    board: Res<Board>,
    mut move_state: ResMut<MoveState>,
    q_chess_cell: Query<&ChessCell>,
    mut q_chess_piece: Query<(&mut ChessPiece, &mut Transform)>,
) {
    if !move_state.move_in_action {
        return;
    }
    if move_state.selected_piece.is_none() {
        return;
    }
    if move_state.selected_cell.is_none() {
        return;
    }

    let chess_cell = q_chess_cell.get(move_state.selected_cell.unwrap()).unwrap();
    let (mut chess_piece, mut transform) = q_chess_piece
        .get_mut(move_state.selected_piece.unwrap())
        .unwrap();

    chess_piece.pos = chess_cell.pos;

    let target_x = board.x_coordinate(chess_piece.pos.i);
    let target_y = board.y_coordinate(chess_piece.pos.j);

    // let current_x = board.x_coordinate(chess_piece.i);
    // let current_y =  board.y_coordinate(chess_piece.j);

    // let velocity = 2.;

    // transform.translation.x += time.delta_seconds() * velocity * (target_x - current_x);
    // transform.translation.y += time.delta_seconds() * velocity * (target_y - current_y);

    // println!("target y {} , current y {}", target_y, transform.translation.y);

    // if transform.translation.x == target_x && transform.translation.y == target_y {
    //     move_state.move_in_action = false;
    //     move_state.selected_cell = None;
    //     move_state.selected_piece = None;
    // }

    transform.translation.x = target_x;
    transform.translation.y = target_y;

    move_state.reset();
}

fn spawn_piece(
    chess_piece: ChessPiece,
    commands: &mut Commands,
    assets: &AssetServer,
    board: &Board,
) {
    let image = load_image(&chess_piece.color, &chess_piece.piece_type, assets);
    commands
        .spawn_bundle(SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::new(
                    board.x_coordinate(chess_piece.pos.i),
                    board.y_coordinate(chess_piece.pos.j),
                    1.0,
                ),
                scale: Vec3::splat(board.image_scale),
                ..default()
            },
            ..Default::default()
        })
        .insert(chess_piece);
}

fn load_image(color: &ChessColor, piece_type: &PieceType, assets: &AssetServer) -> Handle<Image> {
    let color_name = match color {
        ChessColor::WHITE => "w",
        ChessColor::BLACK => "b",
    };

    let type_name = match piece_type {
        PieceType::PAWN => "pawn",
        PieceType::BISHOP => "bishop",
        PieceType::KNIGHT => "knight",
        PieceType::ROOK => "rook",
        PieceType::QUEEN => "queen",
        PieceType::KING => "king",
    };
    let path = format!(
        "shadowed/128px/{}_{}_png_shadow_128px.png",
        color_name, type_name
    );

    return assets.load(&path);
}
