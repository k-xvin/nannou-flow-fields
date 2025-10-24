extern crate nannou;
use nannou::prelude::*;

const GRID_COLS: usize = 20;
const GRID_ROWS: usize = 20;

fn main() {
    println!("hello");
    nannou::app(model).update(update) .simple_window(view).run();
}

struct Model {
    initialized: bool,
    grid: [[f32; GRID_COLS]; GRID_ROWS],
}

fn model(_app: &App) -> Model {
    Model {
        initialized: false,
        grid: [[0.0; GRID_COLS]; GRID_ROWS],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.initialized {
        return;
    }
    model.initialized = true;
    // for row in &mut model.grid {
    //     for radians in row {
    //         *radians += PI/60.0;
    //     }
    // }
    for (row_i, row) in model.grid.iter_mut().enumerate() {
        for (_col_i, radians) in row.iter_mut().enumerate() {
            *radians = ((row_i as f32) / (GRID_ROWS as f32)) * PI;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let cell_w = win.w() / GRID_COLS as f32;
    let cell_h = win.h() / GRID_ROWS as f32;
    let draw = app.draw();

    draw.background().color(WHITE);

    for (row_i, row) in model.grid.iter().enumerate() {
        for (col_i, radians) in row.iter().enumerate() {
            let cell_top_left_x = win.left() + (cell_w * col_i as f32);
            let cell_top_left_y = win.top() - (cell_h * row_i as f32);
            let cell_cx = cell_top_left_x + cell_w / 2.0;
            let cell_cy = cell_top_left_y - cell_h / 2.0;

            // Visualize the center of a cell
            draw.ellipse()
                .x_y(cell_cx, cell_cy)
                .radius(2.0)
                .color(BLACK);

            // Visualize flow direction of the cell
            let arrow_end_x = cell_cx + (cell_w / 2.0) * radians.cos();
            let arrow_end_y = cell_cy + (cell_h / 2.0) * radians.sin();
            draw.line()
                .points(vec2(cell_cx, cell_cy), vec2(arrow_end_x, arrow_end_y))
                .weight(1.0)
                .color(RED);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
