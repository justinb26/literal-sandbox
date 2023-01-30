use nannou::prelude::*;

mod cell_lib;
use cell_lib::{Cell,CellType,World};
use cell_lib::{SAND_CELL};


static WIDTH: i32 = 40;
static HEIGHT: i32 = 40;

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
}

fn model(app: &App) -> Model {
    let mut world = World::new(WIDTH,HEIGHT);

    // Drop some sand for giggles
    for x in 15..22 {
        for y in 0..10 {
                if (rand::random() || rand::random()) {
                    let world_idx = world.get_index(x,y);
                    world.cells[world_idx] = SAND_CELL;
                }
        }
    }
    
    Model { world: world }
}



fn main() {

    nannou::app(model).update(update_model).simple_window(view).run()
}

fn update_model(app: &App, _model: &mut Model, update: nannou::event::Update){
    //println!("UPDATING MODEL");
    _model.world.update();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // // Begin drawing
    
    let win = app.window_rect();
    
    let t = app.time;
    let draw = app.draw();
    
    // Clear the background to black.
    draw.background().color(BLACK);
    
    // Draw a pixel for every cell that is sand
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            
            let cell_type = _model.world.get_cell(x,y).cell_type;
            let xx: f32 = (x * 10) as f32;
            let yy: f32 = (y * 10) as f32;

            if cell_type == CellType::Sand {
                draw.rect().x_y(xx,HEIGHT as f32 - yy).w_h(10.0, 10.0).color(YELLOW);
            } else {
                draw.rect().x_y(xx, HEIGHT as f32 - yy).w_h(10.0, 10.0).color(DARKBLUE);
            }
        }
    }
    
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
