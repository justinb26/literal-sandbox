use crate::cell_lib::*;
use crate::api_lib::*;

// =====================================================
#[repr(C)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        World {
            width: width,
            height: height,
            cells: vec![BLANK_CELL; (width*height) as usize],
        }
    }

    pub fn get_index(&self, x: i32,y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        let cell_idx: usize = (&self).get_index(x,y);
        self.cells[cell_idx]
    }

    pub fn update(&mut self) {
        // Clear updated flag
        for i in 0..self.cells.len() {
            self.cells[i].updated = false;
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x,y);
                World::update_cell(cell, Api { world: self, x, y})
            }
        }            
    }

    fn update_cell(cell: Cell, api: Api) {
        if cell.updated {
            return;
        }

        cell.update(api);
    }
}
