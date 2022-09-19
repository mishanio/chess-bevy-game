use std::ops::Range;

use bevy::prelude::Component;

// mouse pointer resource
pub struct BoardPointer {
    pub x: f32,
    pub y: f32,
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

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct CellPosition {
    pub i: i8,
    pub j: i8,
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

    fn image_size_scaled(&self) -> f32 {
        self.image_size * self.image_scale
    }

    pub fn board_offset(&self) -> f32 {
        let magic = 60.;
        return magic * self.image_scale;
    }

    pub fn x_coordinate(&self, pos: i8) -> f32 {
        self.start_x_point + (pos as f32) * self.image_size_scaled()
    }

    pub fn y_coordinate(&self, pos: i8) -> f32 {
        self.start_y_point + (pos as f32) * self.image_size_scaled()
    }

    pub fn is_cell_matches(&self, pos: &CellPosition, pointer: &BoardPointer) -> bool {
        let ref this = self;
        let size = this.image_size_scaled();
        let x = this.start_x_point + (pos.i as f32) * size;
        let y = this.start_y_point + (pos.j as f32) * size;
        return x < pointer.x && y < pointer.y && (x + size) > pointer.x && (y + size) > pointer.y;
    }
}

#[derive(Component)]
pub struct MainCamera;
