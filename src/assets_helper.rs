use bevy::prelude::*;
use crate::Board;
use crate::models::chess_models::{ChessCell, ChessColor, ChessPiece, PieceType, RemovedChessPiece};
use crate::models::common_resources::DiscardArea;

pub struct AssetsHelper;

impl AssetsHelper {
    pub fn spawn_piece(
        chess_piece: ChessPiece,
        commands: &mut Commands,
        assets: &AssetServer,
        board: &Board,
    ) {
        let image = AssetsHelper::load_piece_image(&chess_piece.color, &chess_piece.piece_type, assets);
        let (x, y) = board.coordinates(&chess_piece.pos);
        commands
            .spawn_bundle(SpriteBundle {
                texture: image,
                transform: Transform {
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::splat(board.image_scale),
                    ..default()
                },
                ..Default::default()
            })
            .insert(chess_piece);
    }

    pub fn spawn_removed_piece(
        chess_piece: RemovedChessPiece,
        commands: &mut Commands,
        assets: &AssetServer,
        board: &Board,
    ) {
        let image = AssetsHelper::load_piece_image(&chess_piece.color, &chess_piece.piece_type, assets);
        let discard_area = match chess_piece.color {
            ChessColor::WHITE => DiscardArea::BOTTOM,
            ChessColor::BLACK => DiscardArea::TOP,
        };
        let (x, y) = board.discard_tray_position(chess_piece.num, &discard_area);
        commands
            .spawn_bundle(SpriteBundle {
                texture: image,
                transform: Transform {
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::splat(board.image_scale),
                    ..default()
                },
                ..Default::default()
            })
            .insert(chess_piece);
    }

    pub fn spawn_chess_cell(cell: ChessCell, commands: &mut Commands, board: &Board, assets: &AssetServer) {
        let (x, y) = board.coordinates(&cell.pos);
        let cell_image = AssetsHelper::load_cell_image(&cell.color(), assets);
        commands
            .spawn_bundle(SpriteBundle {
                texture: cell_image,
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: Vec3::splat(board.image_scale),
                    ..default()
                },
                ..Default::default()
            })
            .insert(cell);
    }


    fn load_piece_image(
        color: &ChessColor,
        piece_type: &PieceType,
        assets: &AssetServer,
    ) -> Handle<Image> {
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



    fn load_cell_image(color: &ChessColor, assets: &AssetServer) -> Handle<Image> {
        let sprite_name = match color {
            ChessColor::WHITE => "square brown light_png_shadow_128px.png",
            ChessColor::BLACK => "square brown dark_png_shadow_128px.png",
        };

        return assets.load(&format!("shadowed/128px/{}", sprite_name));
    }

// fn load_hightlight_cell_image(assets: &AssetServer) -> Handle<Image> {
//     return assets.load("shadowed/128px/square gray light _png_shadow_128px.png");
// }
// fn load_selected_cell_image(assets: &AssetServer) -> Handle<Image> {
//     return assets.load("shadowed/128px/square gray dark _png_shadow_128px.png");
// }
}