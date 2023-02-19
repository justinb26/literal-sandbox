use strum_macros::{IntoStaticStr, Display, EnumIter};
use crate::api_lib::Api;

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoStaticStr, Display, EnumIter)]
#[repr(C)]
pub enum CellType {
    Void,
    #[strum(serialize="Sandy")]
    Sand,
    Stone,
    Mite
}

impl CellType {
    pub fn next(&self) -> CellType {
        match self {
            CellType::Void => CellType::Sand,
            CellType::Sand => CellType::Mite,
            CellType::Mite => CellType::Stone,
            CellType::Stone => CellType::Void,
        }
    }
    
    pub fn prev(&self) -> CellType {
        match self {
            CellType::Void => CellType::Stone,
            CellType::Sand => CellType::Void,
            CellType::Mite => CellType::Sand,
            CellType::Stone => CellType::Mite,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Cell {
    pub cell_type: CellType,
    data1: u8,
    data2: u8,
    pub updated: bool
}

static LEFT: i32 = -1;
static RIGHT: i32 = 1;
// static UP: i32 = -1;
static DOWN: i32 = 1;


impl Cell {
    pub fn update(self, mut api: Api) {


        match self.cell_type {

            CellType::Sand => {
                let down_neighbor = api.get_rel(0, 1);
                if down_neighbor.cell_type == CellType::Void {
                    api.set_rel(0,0,BLANK_CELL);
                    api.set_rel(0,DOWN,self); 
                } else {
                    let dl_neighbor = api.get_rel(LEFT, DOWN);
                    let dr_neighbor = api.get_rel(RIGHT, DOWN);

                    if rand::random() {
                        // Down left first
                        if dl_neighbor.cell_type == CellType::Void && 
                            down_neighbor.cell_type == CellType::Sand {
                            api.set_rel(0,0,BLANK_CELL);
                            api.set_rel(LEFT, DOWN,self); 
                        } else  {
                            if dr_neighbor.cell_type == CellType::Void &&
                            down_neighbor.cell_type == CellType::Sand {
                                api.set_rel(0,0,BLANK_CELL);
                                api.set_rel(RIGHT, DOWN,self); 
                            }
                        }
                    } else {
                        // Down right first
                        if dr_neighbor.cell_type == CellType::Void &&
                            down_neighbor.cell_type == CellType::Sand {
                            api.set_rel(0,0,BLANK_CELL);
                            api.set_rel(1,1,self); 
                        } else if dl_neighbor.cell_type == CellType::Void &&
                            down_neighbor.cell_type == CellType::Sand {
                            api.set_rel(0,0,BLANK_CELL);
                            api.set_rel(-1,1,self); 
                        }
                    }
                }
            },


            CellType::Mite => {
                let down_neighbor = api.get_rel(0, 1);

                // Eat
                if down_neighbor.cell_type == CellType::Sand {
                    api.set_rel(0,0,BLANK_CELL);
                    api.set_rel(0,1,self); 
                } else {
                    let dl_neighbor = api.get_rel(-1, 1);
                    // Down left first
                    if dl_neighbor.cell_type == CellType::Sand {
                        // api.set_rel(0,0,BLANK_CELL);
                        api.set_rel(-1,1,self); 
                    } else  {
                        let dr_neighbor = api.get_rel(1, 1);
                        if dr_neighbor.cell_type == CellType::Sand {
                            // api.set_rel(0,0,BLANK_CELL);
                            api.set_rel(1,1,self); 
                        }
                    }
                }
            },


            CellType::Stone => { },
            
            _ => { return; },


        }
    }
}

pub static BLANK_CELL: Cell = Cell {
    cell_type: CellType::Void,
    data1: 0,
    data2: 0,
    updated: false,
};

pub static SAND_CELL: Cell = Cell {
    cell_type: CellType::Sand,
    data1: 0,
    data2: 0,
    updated: false,
};

pub static STONE_CELL: Cell = Cell {
    cell_type: CellType::Stone,
    data1: 0,
    data2: 0,
    updated: false,
};

pub static MITE_CELL: Cell = Cell {
    cell_type: CellType::Mite,
    data1: 0,
    data2: 0,
    updated: false,
};
