use nannou::prelude::*;
mod cell_lib;
use cell_lib::{CellType,World};
use cell_lib::{SAND_CELL, BLANK_CELL, STONE_CELL, MITE_CELL};

static WIDTH_IN_CELLS: i32 = 80;
static HEIGHT_IN_CELLS: i32 = 80;

//static CELLS_SIZE: f32 = 4.0;

// GOAL: Falling sand with static solids
// Next goal: Water, Quicksand?
// Notable checkpoints:
// Grid system
// Falling particle
// Stacks of sand
// Solid barrier
// Mouse input
// Multiple particle types
// Interacting particles
///////////////////////
// Pause simulation
// GUI
// Resizing
// WASM
// Perf optimization

struct Model {
    world: World,
    cell_width: f32,
    cell_height: f32,
    pos_x: f32,
    pos_y: f32,
    mouse_down: i32,
    tool: CellType,
}

fn model(app: &App) -> Model {
    let world: World = World::new(WIDTH_IN_CELLS, HEIGHT_IN_CELLS);
    app.set_loop_mode(LoopMode::rate_fps(2.0));
    app.new_window().event(event).view(view).build().unwrap();
    
    // calculate width and height in cells
    let win: Rect = app.window_rect();
    let cell_width: f32 = win.w() / WIDTH_IN_CELLS as f32;
    let cell_height: f32 = win.h() / HEIGHT_IN_CELLS as f32;
    
     Model { 
        world,
        cell_width,
        cell_height,
        pos_x: 0.0,
        pos_y: 0.0,
        mouse_down: 0,
        tool: CellType::Sand,
     }
}

fn cell_for_coords(_model: &mut Model, x: f32, y: f32) -> (i32, i32) {

    let new_x = (x / _model.cell_width) as i32;
    let new_y = (y / _model.cell_height) as i32;

    (new_x, new_y)
}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {
    
    match _event {
        MouseMoved(_pos) => {
            if _model.mouse_down == 1 {
    
                let win = _app.window_rect();
                
                // Translate to Top-Left origin
                let xx = clamp(_model.pos_x + (win.w() / 2.0), 0.0, win.w()-1.0);
                let yy = clamp((win.h() / 2.0) - _model.pos_y, 0.0, win.h()-1.0);

                // Get equivalent  X/Y cell position
                let (xxx, yyy) = cell_for_coords(_model, xx, yy);
                println!("{} {}", xxx, yyy);
                
                let world_idx: usize = _model.world.get_index(xxx,yyy);
                _model.world.cells[world_idx] = match _model.tool {
                    CellType::Sand => SAND_CELL,
                    CellType::Void => BLANK_CELL,
                    CellType::Stone => STONE_CELL,
                    CellType::Mite => MITE_CELL,
                };
            }
            
             _model.pos_x = _pos.x;
             _model.pos_y = _pos.y;
        }
        MousePressed(_button) => { 
            
            let win = _app.window_rect();
            
            // Translate to Top-Left origin
            let xx = _model.pos_x + (win.w() / 2.0);
            let yy = (win.h() / 2.0) - _model.pos_y;
            
            let (xxx, yyy) = cell_for_coords(_model, xx, yy);

            // TODO: Ewww, repetition
            let world_idx: usize = _model.world.get_index(xxx,yyy);
            _model.world.cells[world_idx] = match _model.tool {
                CellType::Sand => SAND_CELL,
                CellType::Void => BLANK_CELL,
                CellType::Stone => STONE_CELL,
                CellType::Mite => MITE_CELL,
            };
            _model.mouse_down = 1 as i32;
        }
        MouseReleased(_button) => {
            _model.mouse_down = 0;
        },
        MouseWheel(_scroll_delta, _touch_phase) => {
            let scroll = match _scroll_delta {
                MouseScrollDelta::LineDelta(_x,y) => { y },
                _ => { 0.0 },
            };

            if scroll > 0.0 {
                _model.tool = _model.tool.next();
            }  else if scroll < 0.0 {
                _model.tool = _model.tool.prev();
            }

        }
        _ => {}
    };

    
}


fn main() {
    nannou::app(model).update(update_model).run()
}

fn update_model(_app: &App, _model: &mut Model, _update: nannou::event::Update){
    //println!("UPDATING MODEL");
    _model.world.update();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let win = app.window_rect();
    let _t = app.time;

    let draw = app.draw();
    
    // Set the background to black.
    draw.background().color(BLACK);

    let r = Rect::from_w_h(win.w(),win.h()).top_left_of(win);
 
    // Draw a pixel for every cell that is sand
    for x in 0..WIDTH_IN_CELLS {
        for y in 0..HEIGHT_IN_CELLS {
            let cell_type = _model.world.get_cell(x,y).cell_type;
            
            let xx: f32 = x as f32 * _model.cell_width;
            let yy: f32 = y as f32 * _model.cell_height;

            match cell_type {
                CellType::Sand => { 
                    draw.rect()
                        .x_y(win.left() + xx + _model.cell_width,
                            win.top() - yy
                        )
                        .w_h(_model.cell_width, _model.cell_height)
                        .color(YELLOW);                    
                },
                CellType::Stone => {
                    draw.rect()
                        .x_y(win.left() + xx + _model.cell_width, 
                            win.top() - yy
                        )
                        .w_h(_model.cell_width, _model.cell_height)
                        .color(LIGHTGRAY);                    
                },
                CellType::Mite => {
                    draw.rect()
                    .x_y(win.left() + xx + _model.cell_width, 
                        win.top() - yy
                    )
                    .w_h(_model.cell_width, _model.cell_height)
                    .color(ORANGERED);
                },
                _ => { 
                    // draw.rect().x_y(xx-win.w()/2.0,HEIGHT_IN_CELLS as f32 - yy ).w_h(CELLS_SIZE, CELLS_SIZE).color(BLUE); 
                },
            };
        }
    }
    
    // Draw text
    let tool_string = match _model.tool {
        CellType::Sand => "Sand",
        CellType::Stone => "Stone",
        CellType::Void => "Void",
        CellType::Mite => "Mite",
        // _ => "Sand",
    };
    
    draw.text(tool_string).x_y(r.left()+20.0, r.top()-20.0);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
