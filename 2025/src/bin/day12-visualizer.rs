use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, sleep},
    time::Duration,
};

use advent_of_code_2025::day12::{Input, parse, place, try_place, unplace};
use macroquad::{Window, prelude::*};

enum Event {
    NewRegion {
        w: usize,
        h: usize,
        region_idx: usize,
        total_regions: usize,
        shape_previews: Vec<Vec<(i32, i32)>>,
        counts: Vec<usize>,
    },
    CellCountReject,
    Place {
        shape_type: usize,
        cells: Vec<(usize, usize)>,
    },
    Unplace {
        cells: Vec<(usize, usize)>,
    },
    RegionResult(bool),
    Finished(usize),
}

const SHAPE_COLORS: [Color; 6] = [RED, BLUE, GREEN, YELLOW, PURPLE, ORANGE];

fn solver_thread(input: Input, tx: std::sync::mpsc::Sender<Event>) {
    let mut fit_count = 0;

    for (region_idx, (w, h, counts)) in input.regions.iter().enumerate() {
        let shape_previews: Vec<Vec<(i32, i32)>> = input
            .shapes
            .iter()
            .map(|orients| orients[0].clone())
            .collect();
        let _ = tx.send(Event::NewRegion {
            w: *w,
            h: *h,
            region_idx,
            total_regions: input.regions.len(),
            shape_previews: shape_previews.clone(),
            counts: counts.clone(),
        });
        sleep(Duration::from_millis(200));

        let area = w * h;
        let total_cells: usize = counts
            .iter()
            .zip(input.shape_cells.iter())
            .map(|(&c, &s)| c * s)
            .sum();

        if total_cells > area {
            let _ = tx.send(Event::CellCountReject);
            sleep(Duration::from_millis(300));
            let _ = tx.send(Event::RegionResult(false));
            sleep(Duration::from_millis(200));
            continue;
        }

        let total_shapes: usize = counts.iter().sum();
        if total_shapes == 0 {
            fit_count += 1;
            let _ = tx.send(Event::RegionResult(true));
            sleep(Duration::from_millis(200));
            continue;
        }

        let mut shape_list: Vec<usize> = Vec::new();
        let mut order: Vec<usize> = (0..counts.len()).collect();
        order.sort_by_key(|&i| input.shapes[i].len());
        for &i in &order {
            for _ in 0..counts[i] {
                shape_list.push(i);
            }
        }

        let skip_budget = area - total_cells;
        let mut grid = vec![0u64; *h];

        let result = instrumented_solve(
            &mut grid,
            *w,
            *h,
            &input.shapes,
            &shape_list,
            0,
            skip_budget,
            0,
            &tx,
        );

        if result {
            fit_count += 1;
        }
        let _ = tx.send(Event::RegionResult(result));
        sleep(Duration::from_millis(400));
    }

    let _ = tx.send(Event::Finished(fit_count));
}

#[allow(clippy::too_many_arguments)]
fn instrumented_solve(
    grid: &mut [u64],
    w: usize,
    h: usize,
    shapes: &[Vec<Vec<(i32, i32)>>],
    shape_list: &[usize],
    shape_idx: usize,
    skip_budget: usize,
    search_from: usize,
    tx: &std::sync::mpsc::Sender<Event>,
) -> bool {
    if shape_idx >= shape_list.len() {
        return true;
    }

    let mut idx = search_from;
    while idx < w * h {
        let r = idx / w;
        let c = idx % w;
        if grid[r] & (1u64 << c) == 0 {
            break;
        }
        idx += 1;
    }
    if idx >= w * h {
        return false;
    }
    let r = idx / w;
    let c = idx % w;

    let remaining_cells = w * h - idx;
    let remaining_shape_cells: usize = shape_list[shape_idx..]
        .iter()
        .map(|&s| shapes[s][0].len())
        .sum();
    if remaining_shape_cells > remaining_cells {
        return false;
    }

    let current_type = shape_list[shape_idx];

    for orient in &shapes[current_type] {
        if try_place(grid, w, h, r, c, orient) {
            place(grid, r, c, orient);

            let cells: Vec<(usize, usize)> = orient
                .iter()
                .map(|&(dr, dc)| ((r as i32 + dr) as usize, (c as i32 + dc) as usize))
                .collect();

            let _ = tx.send(Event::Place {
                shape_type: current_type,
                cells: cells.clone(),
            });
            sleep(Duration::from_millis(15));

            if instrumented_solve(
                grid,
                w,
                h,
                shapes,
                shape_list,
                shape_idx + 1,
                skip_budget,
                idx + 1,
                tx,
            ) {
                return true;
            }

            unplace(grid, r, c, orient);
            let _ = tx.send(Event::Unplace { cells });
        }
    }

    if skip_budget > 0
        && instrumented_solve(
            grid,
            w,
            h,
            shapes,
            shape_list,
            shape_idx,
            skip_budget - 1,
            idx + 1,
            tx,
        )
    {
        return true;
    }

    false
}

fn main() {
    let input_str = include_str!("../../input/2025/day12.txt");
    let input = parse(input_str);

    let (tx, rx) = mpsc::channel::<Event>();

    thread::spawn(move || {
        solver_thread(input, tx);
        loop {
            sleep(Duration::from_secs(3600));
        }
    });

    Window::from_config(
        Conf {
            sample_count: 4,
            window_title: "Day12 - Shape Packing Visualizer".to_string(),
            window_height: 800,
            window_width: 1050,
            fullscreen: false,
            high_dpi: false,
            window_resizable: false,
            ..Default::default()
        },
        render_loop(rx),
    );
}

const GRID_AREA: f32 = 800.0;
const SIDEBAR_X: f32 = 820.0;
const SIDEBAR_W: f32 = 220.0;

async fn render_loop(rx: Receiver<Event>) {
    let mut grid: Vec<Vec<Option<usize>>> = vec![];
    let mut grid_w: usize = 0;
    let mut grid_h: usize = 0;
    let mut region_idx: usize = 0;
    let mut total_regions: usize = 0;
    let mut fit_count: usize = 0;
    let mut status = "Waiting...".to_string();
    let mut finished = false;
    let mut result_flash: Option<(bool, f32)> = None;
    let mut reject_flash: f32 = 0.0;
    let mut shape_previews: Vec<Vec<(i32, i32)>> = vec![];
    let mut counts: Vec<usize> = vec![];

    loop {
        while let Ok(event) = rx.try_recv() {
            match event {
                Event::NewRegion {
                    w,
                    h,
                    region_idx: ri,
                    total_regions: tr,
                    shape_previews: sp,
                    counts: c,
                } => {
                    grid_w = w;
                    grid_h = h;
                    grid = vec![vec![None; w]; h];
                    region_idx = ri;
                    total_regions = tr;
                    shape_previews = sp;
                    counts = c;
                    status = format!("Solving region {}/{}", ri + 1, tr);
                    result_flash = None;
                    reject_flash = 0.0;
                }
                Event::CellCountReject => {
                    status = "Cell count mismatch - skipping".to_string();
                    reject_flash = 1.0;
                }
                Event::Place { shape_type, cells } => {
                    for (r, c) in &cells {
                        if *r < grid_h && *c < grid_w {
                            grid[*r][*c] = Some(shape_type);
                        }
                    }
                }
                Event::Unplace { cells } => {
                    for (r, c) in &cells {
                        if *r < grid_h && *c < grid_w {
                            grid[*r][*c] = None;
                        }
                    }
                }
                Event::RegionResult(success) => {
                    if success {
                        fit_count += 1;
                        status = format!("Region {} - FIT!", region_idx + 1);
                    } else {
                        status = format!("Region {} - no fit", region_idx + 1);
                    }
                    result_flash = Some((success, 1.0));
                }
                Event::Finished(count) => {
                    status = format!("Done! {} regions fit", count);
                    finished = true;
                }
            }
        }

        clear_background(Color::new(0.1, 0.1, 0.12, 1.0));

        if grid_w > 0 && grid_h > 0 {
            let cell_size = f32::min(700.0 / grid_w as f32, 700.0 / grid_h as f32);
            let offset_x = (GRID_AREA - cell_size * grid_w as f32) / 2.0;
            let offset_y = 60.0 + (700.0 - cell_size * grid_h as f32) / 2.0;

            for (r, row) in grid.iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    let x = offset_x + c as f32 * cell_size;
                    let y = offset_y + r as f32 * cell_size;

                    match cell {
                        Some(type_idx) => {
                            let color = SHAPE_COLORS[type_idx % SHAPE_COLORS.len()];
                            draw_rectangle(x, y, cell_size, cell_size, color);
                        }
                        None => {
                            draw_rectangle(
                                x,
                                y,
                                cell_size,
                                cell_size,
                                Color::new(0.18, 0.18, 0.22, 1.0),
                            );
                        }
                    }
                }
            }

            let line_color = Color::new(0.3, 0.3, 0.35, 1.0);
            for r in 0..=grid_h {
                let y = offset_y + r as f32 * cell_size;
                draw_line(
                    offset_x,
                    y,
                    offset_x + grid_w as f32 * cell_size,
                    y,
                    1.0,
                    line_color,
                );
            }
            for c in 0..=grid_w {
                let x = offset_x + c as f32 * cell_size;
                draw_line(
                    x,
                    offset_y,
                    x,
                    offset_y + grid_h as f32 * cell_size,
                    1.0,
                    line_color,
                );
            }

            if let Some((success, alpha)) = &mut result_flash {
                let flash_color = if *success {
                    Color::new(0.0, 1.0, 0.0, *alpha * 0.15)
                } else {
                    Color::new(1.0, 0.0, 0.0, *alpha * 0.15)
                };
                draw_rectangle(
                    offset_x,
                    offset_y,
                    grid_w as f32 * cell_size,
                    grid_h as f32 * cell_size,
                    flash_color,
                );
                *alpha -= 0.02;
                if *alpha <= 0.0 {
                    result_flash = None;
                }
            }

            if reject_flash > 0.0 {
                let flash_color = Color::new(1.0, 0.5, 0.0, reject_flash * 0.2);
                draw_rectangle(
                    offset_x,
                    offset_y,
                    grid_w as f32 * cell_size,
                    grid_h as f32 * cell_size,
                    flash_color,
                );
                reject_flash -= 0.02;
            }
        }

        draw_sidebar(&shape_previews, &counts);

        let hud_color = if finished { GREEN } else { LIGHTGRAY };
        draw_text(&status, 10.0, 25.0, 24.0, hud_color);

        if grid_w > 0 && grid_h > 0 {
            draw_text(
                &format!("{}x{}", grid_w, grid_h),
                10.0,
                48.0,
                20.0,
                DARKGRAY,
            );
        }

        let fits_text = format!("Fits: {}", fit_count);
        let region_text = format!("Region {}/{}", region_idx + 1, total_regions);
        let fits_width = measure_text(&fits_text, None, 24, 1.0).width;
        let region_width = measure_text(&region_text, None, 20, 1.0).width;
        draw_text(&fits_text, GRID_AREA - 10.0 - fits_width, 25.0, 24.0, GREEN);
        draw_text(
            &region_text,
            GRID_AREA - 10.0 - region_width,
            48.0,
            20.0,
            DARKGRAY,
        );

        next_frame().await;
    }
}

fn draw_sidebar(shape_previews: &[Vec<(i32, i32)>], counts: &[usize]) {
    let preview_cell = 12.0_f32;
    let mut y_cursor = 60.0;

    draw_text("Shapes", SIDEBAR_X, y_cursor, 22.0, LIGHTGRAY);
    y_cursor += 10.0;

    draw_line(
        SIDEBAR_X,
        y_cursor,
        SIDEBAR_X + SIDEBAR_W - 10.0,
        y_cursor,
        1.0,
        DARKGRAY,
    );
    y_cursor += 15.0;

    for (type_idx, cells) in shape_previews.iter().enumerate() {
        let color = SHAPE_COLORS[type_idx % SHAPE_COLORS.len()];
        let count = counts.get(type_idx).copied().unwrap_or(0);

        let max_r = cells.iter().map(|&(r, _)| r).max().unwrap_or(0) + 1;

        let dimmed = if count == 0 {
            Color::new(color.r, color.g, color.b, 0.25)
        } else {
            color
        };
        draw_text(
            &format!("x{}", count),
            SIDEBAR_X,
            y_cursor + (max_r as f32 * preview_cell) / 2.0 + 5.0,
            18.0,
            dimmed,
        );

        let shape_x = SIDEBAR_X + 35.0;
        for &(r, c) in cells {
            let x = shape_x + c as f32 * preview_cell;
            let y = y_cursor + r as f32 * preview_cell;
            draw_rectangle(
                x + 1.0,
                y + 1.0,
                preview_cell - 2.0,
                preview_cell - 2.0,
                dimmed,
            );
        }

        y_cursor += max_r as f32 * preview_cell + 12.0;
    }
}
