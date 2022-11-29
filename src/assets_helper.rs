use crate::models::chess_cell::ChessCell;
use crate::models::chess_piece::{ChessPiece, PieceType};
use crate::models::common_chess::ChessColor;
use crate::models::common_resources::{DiscardArea, FontHolder, StaticDespawnable};
use crate::models::removed_chess_piece::RemovedChessPiece;
use crate::Board;
use bevy::prelude::*;

pub struct AssetsHelper;

impl AssetsHelper {
    pub fn spawn_piece(
        commands: &mut Commands,
        chess_piece: ChessPiece,
        vec3: Vec3,
        assets: &AssetServer,
        board: &Board,
    ) {
        let image =
            AssetsHelper::load_piece_image(&chess_piece.color, &chess_piece.piece_type, assets);

        commands
            .spawn(SpriteBundle {
                texture: image,
                transform: Transform {
                    translation: vec3,
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
        let image =
            AssetsHelper::load_piece_image(&chess_piece.color, &chess_piece.piece_type, assets);
        let discard_area = match chess_piece.color {
            ChessColor::WHITE => DiscardArea::TOP,
            ChessColor::BLACK => DiscardArea::BOTTOM,
        };
        let (x, y) = board.discard_tray_position(chess_piece.num, &discard_area);
        commands
            .spawn(SpriteBundle {
                texture: image,
                transform: Transform {
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::splat(board.discard_image_scale()),
                    ..default()
                },
                ..Default::default()
            })
            .insert(chess_piece);
    }

    pub fn spawn_chess_cell(
        commands: &mut Commands,
        cell: ChessCell,
        vec3: Vec3,
        board: &Board,
        assets: &AssetServer,
    ) {
        let cell_image = AssetsHelper::load_cell_image(&cell.color(), assets);
        commands
            .spawn(SpriteBundle {
                texture: cell_image,
                transform: Transform {
                    translation: vec3,
                    scale: Vec3::splat(board.image_scale),
                    ..default()
                },
                ..Default::default()
            })
            .insert(cell)
            .insert(StaticDespawnable);
    }

    pub fn spawn_chess_boarding_cell(
        commands: &mut Commands,
        vec3: Vec3,
        assets: &AssetServer,
        board: &Board,
    ) {
        let cell_image = AssetsHelper::load_boarding_image(assets);
        commands
            .spawn(SpriteBundle {
                texture: cell_image,
                transform: Transform {
                    translation: vec3,
                    scale: Vec3::splat(board.image_scale),
                    ..default()
                },
                ..Default::default()
            })
            .insert(StaticDespawnable);
    }

    pub fn spawn_text_boarding(
        commands: &mut Commands,
        vec3: Vec3,
        text: String,
        font_holder: &FontHolder,
        board: &Board,
    ) {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: font_holder.font.clone(),
                        font_size: board.image_size_scaled() / 2.,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::CENTER),
                transform: Transform {
                    translation: vec3,
                    ..default()
                },
                ..default()
            })
            .insert(StaticDespawnable);
    }

    pub fn load_piece_image(
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

    fn load_boarding_image(assets: &AssetServer) -> Handle<Image> {
        let sprite_name = "square gray light _png_shadow_128px.png";
        return assets.load(&format!("shadowed/128px/{}", sprite_name));
    }

    // fn load_hightlight_cell_image(assets: &AssetServer) -> Handle<Image> {
    //     return assets.load("shadowed/128px/square gray light _png_shadow_128px.png");
    // }
    // fn load_selected_cell_image(assets: &AssetServer) -> Handle<Image> {
    //     return assets.load("shadowed/128px/square gray dark _png_shadow_128px.png");
    // }
}
