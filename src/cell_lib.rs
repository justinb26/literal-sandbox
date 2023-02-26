use rand::Rng;
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
        let down_neighbor = api.get_rel(0, DOWN);
        let dl_neighbor = api.get_rel(LEFT, DOWN);
        let dr_neighbor = api.get_rel(RIGHT, DOWN);

        let mut rng = rand::thread_rng();
        match self.cell_type {
            CellType::Sand => {

                // 1 in 3 chance that the cell will try to move diagonally
                
                let rand_num = rng.gen_range(0..3);

                if rand_num == 0 {
                    let rand_bool = rng.gen_bool(0.5);

                    if rand_bool {
                            
                        // Try to move diagonally, starting with left
                        if dl_neighbor.cell_type == CellType::Void && 
                            down_neighbor.cell_type == CellType::Void {
                            api.set_rel(0,0,BLANK_CELL);
                            api.set_rel(LEFT, DOWN,self); 
                        } else  {
                            if dr_neighbor.cell_type == CellType::Void &&
                            down_neighbor.cell_type == CellType::Void {
                                api.set_rel(0,0,BLANK_CELL);
                                api.set_rel(RIGHT, DOWN,self); 
                            }
                        }
                    } else {
                        // Try to move diagonally, starting with right
                        if dr_neighbor.cell_type == CellType::Void &&
                            down_neighbor.cell_type == CellType::Void {
                            api.set_rel(0,0,BLANK_CELL);
                            api.set_rel(RIGHT, DOWN,self); 
                        } else  {
                            if dl_neighbor.cell_type == CellType::Void && 
                            down_neighbor.cell_type == CellType::Void {
                                api.set_rel(0,0,BLANK_CELL);
                                api.set_rel(LEFT, DOWN,self); 
                            }
                        }
                    }
                } else {

                    
                    if down_neighbor.cell_type == CellType::Void {
                        api.set_rel(0,0,BLANK_CELL);
                        api.set_rel(0,DOWN,self); 
                    } else {
                        let rand_bool = rng.gen_bool(0.5);

                        if rand_bool {
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
                                api.set_rel(RIGHT, DOWN,self); 
                            } else  {
                                if dl_neighbor.cell_type == CellType::Void && 
                                down_neighbor.cell_type == CellType::Sand {
                                    api.set_rel(0,0,BLANK_CELL);
                                    api.set_rel(LEFT, DOWN,self); 
                                }
                            }
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

// Definition of CellBehavior<T> superclass, does not need to contain next or prev
trait CellBehavior<T> {
    fn update(&mut self, _api: Api);
}

// Implement CellBehavior<T> for CellType
impl CellBehavior<Cell> for CellType {
    fn update(&mut self, _api: Api) {}
}

trait Falls<T> : CellBehavior<T>{
    fn update(&mut self, _api: Api);
}

impl<T> Falls<T> for CellType
where T:CellBehavior<T>, CellType:CellBehavior<T>
 {
    fn update(&mut self, _api: Api) {
        println!("Falls!");
    }
}

// create trait and default implementation for Solid<T>
trait Solid<T> : CellBehavior<T>{
    fn update(&mut self, _api: Api) {
        println!("Solid!");
    }
}

impl<T> CellBehavior<T> for CellType
where T:
    CellBehavior<T>, 
    CellType:CellBehavior<T>
 {
    fn update(&mut self, _api: Api) {
        println!("Solid!");
    }
}

// Static cell definitions

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
