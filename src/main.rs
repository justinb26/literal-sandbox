use nannou::prelude::*;
// GOAL: Falling sand with static solids
// Next goal: Water, Quicksand?

// Notable checkpoints:
// Grid system
// Falling particle
// Solid barrier
// Mouse input
// Multiple particle types
// Interacting particles
// GUI
// Resizing
// WASM
// Perf optimization

struct Model {}
fn model(_app: &App) -> Model {
    Model {}
}

fn main() {
    nannou::app(model).simple_window(view).run()
}


fn view(app: &App, _model: &Model, frame: Frame) {
    // // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    // Clear the background to black.
    draw.background().color(BLACK);
    
    draw.ellipse().color(STEELBLUE).w(300.0).h(200.0).x_y(200.0, -100.0);

    // // Random set of points within radius
    // // Spawn a circle of random size at those points
    // let radius = win.w().min(win.h()) * 0.50;
    // let foo: f32 = map_range(t%6.0, 0.0, 5.0, -win.w()-radius, win.w()+radius);
    // draw.ellipse().x(foo-win.w()).color(WHITE).radius(radius);
    
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
 