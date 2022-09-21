use std::borrow::BorrowMut;

use bevy::prelude::*;

use crate::{
    models::chess_models::{ChessCell, ChessCellState, ChessColor, ChessPiece, ChessPieceRemovedEvent, MoveState, PieceType},
    models::common_resources::{Board, BoardPointer},
};
use crate::assets_helper::AssetsHelper;

pub struct ChessBoardPlugin;

impl Plugin for ChessBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, set_up_resources)
            .add_startup_system_to_stage(StartupStage::Startup, set_up_chess_board)
            .add_startup_system_to_stage(StartupStage::Startup, set_up_chess_pieces_system)
            .add_event::<ChessPieceRemovedEvent>()
            .add_system(highlight_chess_piece_system)
            .add_system(calculate_chess_cell_state_system)
            .add_system(draw_highlight_chess_cell_system)
            .add_system(set_piece_selected)
            .add_system(set_cell_selected)
            .add_system(remove_taken_piece_system)
            .add_system(move_piece_system);
    }
}

fn set_up_resources(mut commands: Commands) {
    commands.insert_resource(MoveState::default());
}

fn set_up_chess_board(assets: Res<AssetServer>, mut commands: Commands, board: Res<Board>) {
    for j in board.cell_range() {
        for i in board.cell_range() {
            let cell = ChessCell::from(i, j);
            AssetsHelper::spawn_chess_cell(cell, &mut commands, &board, &assets);
        }
    }
}

fn set_up_chess_pieces_system(assets: Res<AssetServer>, mut commands: Commands, board: Res<Board>) {
    let first = board.first_element;
    let last = board.last_element;
    AssetsHelper::spawn_piece(
        ChessPiece::new(first, last, ChessColor::BLACK, PieceType::ROOK),
        commands.borrow_mut(),
        &assets,
        &board,
    );

    AssetsHelper::spawn_piece(
        ChessPiece::new(first + 3, first + 1, ChessColor::WHITE, PieceType::PAWN),
        commands.borrow_mut(),
        &assets,
        &board,
    );
    AssetsHelper::spawn_piece(
        ChessPiece::new(first + 4, first + 1, ChessColor::WHITE, PieceType::PAWN),
        commands.borrow_mut(),
        &assets,
        &board,
    );
    AssetsHelper::spawn_piece(
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

fn calculate_chess_cell_state_system(
    mut q_chess_cells: Query<&mut ChessCell>,
    board_pointer: Res<BoardPointer>,
    move_state: Res<MoveState>,
    q_chess_piece: Query<&ChessPiece>,
    board: Res<Board>,
) {
    if move_state.move_in_action {
        return;
    }
    if move_state.selected_piece.is_none() {
        for mut chess_cell in q_chess_cells.iter_mut() {
            chess_cell.state = ChessCellState::NONE;
        }
        return;
    }

    let selected_piece = q_chess_piece
        .get(move_state.selected_piece.unwrap()).ok();
    if selected_piece.is_none() {
        return;
    }
    let selected_piece = selected_piece.unwrap();

    let pieces: Vec<&ChessPiece> = q_chess_piece.iter().collect();

    let available_cells = selected_piece.get_available_cells_for_move(&board, &pieces);

    for mut chess_cell in q_chess_cells.iter_mut() {
        let is_current_cell_selected = selected_piece.pos == chess_cell.pos;
        if is_current_cell_selected {
            chess_cell.state = ChessCellState::NONE;
            continue;
        }

        if board.is_cell_matches(&chess_cell.pos, &board_pointer) {
            let is_enemy_piece_selected = pieces
                .iter()
                .find(|cp| cp.pos == chess_cell.pos && selected_piece.color != cp.color).is_some();

            if is_enemy_piece_selected && available_cells.contains(&chess_cell.pos) {
                info!("chess_sell is attacked");
                chess_cell.state = ChessCellState::ATTACKED;
            } else {
                info!("chess_sell is selected");
                chess_cell.state = ChessCellState::SELECTED;
            }
        } else if available_cells.contains(&chess_cell.pos) {
            chess_cell.state = ChessCellState::HIGHLIGHTED;
        } else {
            chess_cell.state = ChessCellState::NONE;
        }
    }
}

fn draw_highlight_chess_cell_system(mut q_chess_cells: Query<(&mut Sprite, &ChessCell)>) {
    for (mut sprite, chess_cell) in q_chess_cells.iter_mut() {
        match chess_cell.state {
            ChessCellState::NONE => sprite.color.set_r(1.),
            ChessCellState::HIGHLIGHTED => sprite.color.set_r(0.7),
            ChessCellState::SELECTED => sprite.color.set_r(0.5),
            ChessCellState::ATTACKED => sprite.color.set_r(0.0),
        };
    }
}

fn set_piece_selected(
    buttons: Res<Input<MouseButton>>,
    pointer: Res<BoardPointer>,
    board: Res<Board>,
    mut move_sate: ResMut<MoveState>,
    q_chess_piece: Query<(Entity, &ChessPiece)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    if move_sate.move_in_action {
        return;
    }

    for (entity, piece) in q_chess_piece.iter() {
        if board.is_cell_matches(&piece.pos, &pointer) && move_sate.current_collor == piece.color {
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
    mut piece_taken_event_writer: EventWriter<ChessPieceRemovedEvent>,
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
        .get(move_state.selected_piece.unwrap()).ok();
    if selected_piece.is_none() {
        return;
    }
    let selected_piece = selected_piece.unwrap();
    let pieces: Vec<&ChessPiece> = q_chess_piece.iter().collect();

    let available_cells = selected_piece.get_available_cells_for_move(&board, &pieces);

    for (entity, cell) in q_chess_cell.iter() {
        let is_current_cell_selected = selected_piece.pos == cell.pos;
        if is_current_cell_selected {
            continue;
        }
        if board.is_cell_matches(&cell.pos, &pointer) && available_cells.contains(&cell.pos) {
            debug!("move_in action set to true");
            if let Some(piece_to_remove) = pieces
                .iter()
                .find(|chess_piece| chess_piece.pos == cell.pos)
            {
                piece_taken_event_writer.send(ChessPieceRemovedEvent {
                    chess_piece: ChessPiece {
                        pos: piece_to_remove.pos,
                        color: piece_to_remove.color.clone(),
                        piece_type: piece_to_remove.piece_type.clone(),
                    }
                });
            }

            move_state.selected_cell = Some(entity);
            move_state.move_in_action = true;
        }
    }
}

fn move_piece_system(
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

    let (target_x, target_y) = board.coordinates(&chess_piece.pos);
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

    move_state.next_move();
}

fn remove_taken_piece_system(
    mut commands: Commands,
    q_chess_piece: Query<(Entity, &ChessPiece)>,
    mut piece_taken_event_reader: EventReader<ChessPieceRemovedEvent>,
) {
    piece_taken_event_reader.iter().for_each(|event| {
        q_chess_piece.iter().for_each(|(entity, cp)| {
            if cp.pos == event.chess_piece.pos && cp.color == event.chess_piece.color {
                commands.entity(entity).despawn();
            }
        })
    })
}


