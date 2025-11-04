use nannou::prelude::*;
use nannou::noise::*;

const GRID_COLS: usize = 200;
const GRID_ROWS: usize = 200;
const NUM_POINTS: usize = 4000;
const NUM_STEPS: usize = 100;
const STEP_LEN: f32 = 1.0;

fn main() {
    println!("hello");
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    noise: Perlin,
    grid: [[f32; GRID_COLS]; GRID_ROWS], // Assumes bottom left is 0,0 for the grid
    lines: Vec<[Point2; NUM_STEPS]>,
    cell_w: f32,
    cell_h: f32,
}

fn model(app: &App) -> Model {
    let noise = Perlin::new().set_seed(1);

    let mut grid = [[0.0; GRID_COLS]; GRID_ROWS];
    for (row_i, row) in grid.iter_mut().enumerate() {
        for (col_i, radians) in row.iter_mut().enumerate() {
            let x = (col_i as f64) * 0.005;
            let y = (row_i as f64) * 0.005;
            let noise_val = noise.get([x, y]) as f32;
            // Noise is between -1.0 and 1.0, so scale it to a radian value between -2PI and 2PI
            *radians = noise_val * 2.0 * PI;
        } 
    }

    let win = app.window_rect();
    let mut lines = Vec::new();
    for _ in 0..NUM_POINTS {
        let start_x = win.left() + random::<f32>() * win.w();
        let start_y = win.bottom() + random::<f32>() * win.h();
        let mut line = [pt2(0.0, 0.0); NUM_STEPS];
        line[0] = pt2(start_x, start_y);
        lines.push(line);
    }

    let cell_w = win.w() / GRID_COLS as f32;
    let cell_h = win.h() / GRID_ROWS as f32;

    Model {
        noise,
        grid,
        lines,
        cell_w,
        cell_h,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();

    // Update flowfield vectors
    for (row_i, row) in model.grid.iter_mut().enumerate() {
        for (col_i, radians) in row.iter_mut().enumerate() {
            let x = (col_i as f64) * 0.005;
            let y = (row_i as f64) * 0.005;
            let noise_val = model.noise.get([x, y, app.elapsed_frames() as f64 * 0.005]) as f32;
            // Noise is between -1.0 and 1.0, so scale it to a radian value between -2PI and 2PI
            *radians = noise_val * 2.0 * PI;
        } 
    }

    // Calculate new positions of each line to draw since flowfield's vectors are now updated
    for line in &mut model.lines {
        for step in 1..NUM_STEPS {
            let last_point = line[step - 1];

            // Shift coordinates so bottom left is (0, 0) in order to determine the position in the grid
            let row = (((last_point.y + (win.h() / 2.0)) / model.cell_h) as usize).min(GRID_ROWS - 1);
            let col = (((last_point.x + (win.w() / 2.0)) / model.cell_w) as usize).min(GRID_COLS - 1);

            // Get the angle at (row, column)
            let angle = model.grid[row][col];

            // Step the line in the direction of the angle
            let next_point = last_point + pt2(STEP_LEN * angle.cos(), STEP_LEN * angle.sin());

            // Save new point
            line[step] = next_point;
        }
    }
    return;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();

    draw.background().color(WHITE);

    // visualize_flowfield(&draw, model, &win, model.cell_w, model.cell_h);

    // Draw each line in the flowfield
    for line in &model.lines {
        // .copied() is an iterator that creates an owned copy of each Point2 as it iterates
        draw.polyline().weight(1.0).caps_round().points(line.iter().copied());
    }

    draw.to_frame(app, &frame).unwrap();
}

/// Draw arrows showing the direction of each point in the flow field
#[allow(dead_code)]
fn visualize_flowfield(draw: &Draw, model: &Model, win: &Rect, cell_w: f32, cell_h: f32) {
    for (row_i, row) in model.grid.iter().enumerate() {
        for (col_i, radians) in row.iter().enumerate() {
            // Create grid where bottom left is 0,0
            let cell_bottom_left_x = win.left() + (cell_w * col_i as f32);
            let cell_bottom_left_y = win.bottom() + (cell_h * row_i as f32);
            let cell_cx = cell_bottom_left_x + cell_w / 2.0;
            let cell_cy = cell_bottom_left_y + cell_h / 2.0;

            // Visualize the center of a cell
            draw.ellipse()
                .x_y(cell_cx, cell_cy)
                .radius(2.0)
                .color(BLACK);

            // Visualize flow direction of the cell
            let arrow_end_x = cell_cx + (cell_w / 2.0) * radians.cos();
            let arrow_end_y = cell_cy + (cell_h / 2.0) * radians.sin();
            draw.line()
                .points(pt2(cell_cx, cell_cy), pt2(arrow_end_x, arrow_end_y))
                .weight(1.0)
                .color(RED);
        }
    }
}
