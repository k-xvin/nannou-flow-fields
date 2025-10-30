use nannou::prelude::*;

const GRID_COLS: usize = 20;
const GRID_ROWS: usize = 20;

fn main() {
    println!("hello");
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    grid: [[f32; GRID_COLS]; GRID_ROWS], // Assumes bottom left is 0,0 for the grid
    points: Vec<Vec2>,
}

fn model(app: &App) -> Model {
    let mut grid = [[0.0; GRID_COLS]; GRID_ROWS];
    for (row_i, row) in grid.iter_mut().enumerate() {
        for (_col_i, radians) in row.iter_mut().enumerate() {
            *radians = ((row_i as f32) / (GRID_ROWS as f32)) * PI;
        }
    }

    let win = app.window_rect();
    let mut points = Vec::new();
    for _ in 0..100 {
        let x = win.left() + random::<f32>() * win.w();
        let y = win.bottom() + random::<f32>() * win.h();
        points.push(pt2(x, y));
    }

    Model {
        grid,
        points,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for row in &mut _model.grid {
        for radians in row {
            *radians += PI/60.0;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let cell_w = win.w() / GRID_COLS as f32;
    let cell_h = win.h() / GRID_ROWS as f32;
    let draw = app.draw();

    draw.background().color(WHITE);

    visualize_flowfield(&draw, model, &win, cell_w, cell_h);

    const NUM_STEPS: usize = 100;
    let mut points_to_draw = [pt2(0.0, 0.0); NUM_STEPS];
    let step_len = 1.0;
    for p in &model.points {
        // Set up all points to draw, with p as the "start" point for the line
        points_to_draw[0] = *p; // Copy contents

        // Calculate all the points for this line
        for step in 1..NUM_STEPS {
            let last_point = points_to_draw[step-1];
            // Shift coordinates so bottom left is (0, 0) in order to determine the position in the grid
            let row = (((last_point.y + (win.h() / 2.0)) / cell_h) as usize).min(GRID_ROWS - 1);
            let col = (((last_point.x + (win.w() / 2.0)) / cell_w) as usize).min(GRID_COLS - 1);

            // Get the angle at (row, column)
            let angle = model.grid[row][col];

            // Step the line in the direction of the angle
            let next_point = last_point + pt2(step_len * angle.cos(), step_len * angle.sin());

            // Save new point
            points_to_draw[step] = next_point;
        }

        // Draw the polyline
        draw.polyline().weight(3.0).points(points_to_draw);
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
