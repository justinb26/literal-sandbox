#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Void,
    Sand,
    Stone,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub cell_type: CellType,
    data1: u8,
    data2: u8,
    data3: u8
}

impl Cell {
    pub fn update(self, cell: Cell, mut api: Api) {
        if cell.cell_type == CellType::Void {
            return;
        }

        let down_neighbor = api.get_rel(0, 1);
        if down_neighbor.cell_type == CellType::Void {
            api.set_rel(0,0,BLANK_CELL);
            api.set_rel(0,1,cell); 
        } else {
            let dl_neighbor = api.get_rel(-1, 1);
            // Down left first
            if dl_neighbor.cell_type == CellType::Void {
                api.set_rel(0,0,BLANK_CELL);
                api.set_rel(-1,1,cell); 
            } else  {
                let dr_neighbor = api.get_rel(1, 1);
                if dr_neighbor.cell_type == CellType::Void {
                    api.set_rel(0,0,BLANK_CELL);
                    api.set_rel(1,1,cell); 
                }
            }
        }
       //  } //else {
        //         // Down right first
        //         if dr_neighbor.cell_type == CellType::Void {
        //             api.set_rel(0,0,BLANK_CELL);
        //             api.set_rel(-1,1,cell); 
        //         } else if dl_neighbor.cell_type == CellType::Void {
        //             api.set_rel(0,0,BLANK_CELL);
        //             api.set_rel(1,1,cell); 
        //         }
        // }

    }
}

pub static BLANK_CELL: Cell = Cell {
    cell_type: CellType::Void,
    data1: 0,
    data2: 0,
    data3: 0,
};

pub static SAND_CELL: Cell = Cell {
    cell_type: CellType::Sand,
    data1: 0,
    data2: 0,
    data3: 0,
};

pub static STONE_CELL: Cell = Cell {
    cell_type: CellType::Stone,
    data1: 0,
    data2: 0,
    data3: 0,
};
// =====================================================
pub struct World {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
}

pub struct Api<'a> {
    pub x: i32,
    pub y: i32,
    pub world: &'a mut World,
}

// =====================================================

impl<'a> Api<'a> {
    pub fn get_rel(&self, dx: i32, dy: i32) -> Cell{

        let x = self.x + dx;
        let y = self.y + dy;

        if x < 0 || x >= self.world.width || y < 0 || y >= self.world.height {
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
        self.world.cells[idx].data3 = 1; // Mark as updated
    }
}

// =====================================================

impl World {
    pub fn new(width: i32, height: i32) -> World {
        World {
            width: width,
            height: height,
            cells: vec![BLANK_CELL; (width*height) as usize],
        }
    }

    // pub fn reset(&mut self) {
    //     for x in 0..self.width {
    //         for y in 0..self.height {
    //             let idx = self.get_index(x, y);
    //             self.cells[idx] = BLANK_CELL;
    //         }
    //     }
    // }

    pub fn get_index(&self, x: i32,y: i32) -> usize {
        // inverted?
        (y * self.width + x) as usize
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        let cell_idx: usize = (&self).get_index(x,y);
        self.cells[cell_idx]
    }

    pub fn update(&mut self) {
        // loop through all cells
        for i in 0..self.cells.len() {
            self.cells[i].data3 = 0;
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x,y);
                if cell.data3 == 0 { // Not yet updated
                    World::update_cell(cell, Api { world: self, x, y})
                }
            }
        }            
        // update cell by trying to move down or diagonally down
    }

    fn update_cell(cell: Cell, mut api: Api) {
        cell.update(cell, api);
    }
}
