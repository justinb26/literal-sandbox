use rand::Rng;

use crate::world_lib::*;
use crate::cell_lib::*;

#[repr(C)]
pub struct Api<'a> {
    pub x: i32,
    pub y: i32,
    pub world: &'a mut World,
}

// =====================================================

impl<'a> Api<'a> {
    pub fn get_rel(&self, dx: i32, dy: i32) -> Cell{
        let (x, y) = (self.x + dx, self.y + dy);

        if x < 0 || y < 0 || x >= self.world.width || y >= self.world.height {
            return STONE_CELL;
        }

        let idx = self.world.get_index(x, y);
        self.world.cells[idx]
    }   

   pub fn set_rel(&mut self, dx: i32, dy:i32, cell: Cell) {
        let x = self.x + dx;
        let y = self.y + dy;
        let idx = self.world.get_index(x, y);

        self.world.cells[idx] = cell;
        self.world.cells[idx].updated = true;
    }

    pub fn swap_cell(&mut self, cell: Cell, x: i32, y: i32) {
        // Get the cell at the given x and y
        let swap_idx = self.world.get_index(self.x+x, self.y+y);
        let swap_cell = self.world.cells[swap_idx];

        let cell_idx = self.world.get_index(self.x, self.y);
        
        // Swap the cells
        self.world.cells[swap_idx] = cell;
        self.world.cells[cell_idx] = swap_cell;

        self.world.cells[cell_idx].updated = true;
        self.world.cells[swap_idx].updated = true;
        
    }
     

}

// =====================================================
