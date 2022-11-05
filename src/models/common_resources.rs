use std::ops::Range;

use bevy::{prelude::{Component, Handle}, text::Font};

pub enum GameState {
    NEW,
    CONTINUE,
}

pub struct FontHolder {
    pub font: Handle<Font>, 
}

// determ position of discard removed chess pieces
pub enum DiscardArea {
    TOP,
    BOTTOM,
}

// mouse pointer resource
pub struct BoardPointer {
    pub x: f32,
    pub y: f32,
}

// position of a cell on board
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct CellPosition {
    pub i: i8,
    pub j: i8,
}

// board control resource
pub struct Board {
    pub start_x_point: f32,
    pub start_y_point: f32,
    pub image_size: f32,
    pub image_scale: f32,
    pub first_element: i8,
    pub last_element: i8,
}

impl Board {
    pub fn new(x: f32, y: f32, i_size: f32, i_scale: f32) -> Board {
        Board {
            start_x_point: x,
            start_y_point: y,
            image_size: i_size,
            image_scale: i_scale,
            first_element: 0,
            last_element: 7,
        }
    }

    pub fn cell_range(&self) -> Range<i8> {
        return self.first_element..self.last_element + 1;
    }

    pub fn image_size_scaled(&self) -> f32 {
        self.image_size * self.image_scale
    }

    pub fn board_offset(&self) -> f32 {
        let magic = 60.;
        return magic * self.image_scale;
    }

    pub fn coordinates(&self, pos: &CellPosition) -> (f32, f32) {
        return (self.x_coordinate(pos.i), self.y_coordinate(pos.j));
    }

    fn x_coordinate(&self, pos: i8) -> f32 {
        self.start_x_point + (pos as f32) * self.image_size_scaled()
    }

    fn y_coordinate(&self, pos: i8) -> f32 {
        self.start_y_point + (pos as f32) * self.image_size_scaled()
    }

    pub fn is_cell_out_of_range(&self, cell: &CellPosition) -> bool {
        return self.is_out_of_range(cell.i) || self.is_out_of_range(cell.j);
    }
    fn is_out_of_range(&self, pos: i8) -> bool {
        pos < self.first_element || pos > self.last_element
    }

    pub fn is_cell_matches(&self, pos: &CellPosition, pointer: &BoardPointer) -> bool {
        let ref this = self;
        let size = this.image_size_scaled();
        let x = this.start_x_point + (pos.i as f32) * size;
        let y = this.start_y_point + (pos.j as f32) * size;
        return x < pointer.x && y < pointer.y && (x + size) > pointer.x && (y + size) > pointer.y;
    }

    pub fn end_x_point(&self) -> f32 {
        self.start_x_point + (self.last_element as f32 + 1.) * self.image_size_scaled()
    }

    pub fn end_y_point(&self) -> f32 {
        self.start_y_point + (self.last_element as f32 + 1.) * self.image_size_scaled()
    }

    //todo move to separate  struct DiscardTray
    pub fn discard_tray_position(&self, element_num: i8, position: &DiscardArea) -> (f32, f32) {
        let board_discard_tray_offset = 1.;
        let direction_coefficient = match position {
            DiscardArea::TOP => 1.,
            DiscardArea::BOTTOM => -1.,
        };
        let discard_start_y_offset = match position {
            DiscardArea::TOP => self.end_y_point() + self.discard_image_size_scaled(),
            DiscardArea::BOTTOM => {
                self.start_y_point
                    - board_discard_tray_offset * self.image_size_scaled()
                    - self.discard_image_size_scaled()
            }
        };

        let y_coordinate = discard_start_y_offset
            + direction_coefficient
                * (element_num / self.last_element) as f32
                * self.discard_image_size_scaled();
        let x_coordinate = self.start_x_point
            + (element_num % self.last_element) as f32 * self.discard_image_size_scaled();

        return (x_coordinate, y_coordinate);
    }

    pub fn discard_image_scale(&self) -> f32 {
        self.image_scale * 0.8
    }

    fn discard_image_size_scaled(&self) -> f32 {
        self.image_size * self.discard_image_scale()
    }
}

#[derive(Component)]
pub struct MainCamera;
