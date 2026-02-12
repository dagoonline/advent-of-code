use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    /// shapes[type_idx][orient_idx] = cells relative to anchor (sorted raster order)
    shapes: Vec<Vec<Vec<(i32, i32)>>>,
    /// (width, height, counts_per_shape_type)
    regions: Vec<(usize, usize, Vec<usize>)>,
    /// number of # cells per shape type
    shape_cells: Vec<usize>,
}

fn normalize(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let min_r = cells.iter().map(|&(r, _)| r).min().unwrap();
    let min_c = cells.iter().map(|&(_, c)| c).min().unwrap();
    let mut result: Vec<(i32, i32)> = cells.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect();

    result.sort();
    result
}

fn rotate_cw(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    normalize(&cells.iter().map(|&(r, c)| (c, -r)).collect::<Vec<_>>())
}

fn flip_h(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    normalize(&cells.iter().map(|&(r, c)| (r, -c)).collect::<Vec<_>>())
}

fn all_orientations(cells: &[(i32, i32)]) -> Vec<Vec<(i32, i32)>> {
    let mut result = Vec::new();
    let mut current = normalize(cells);

    for _ in 0..4 {
        if !result.contains(&current) {
            result.push(current.clone());
        }

        let f = flip_h(&current);
        if !result.contains(&f) {
            result.push(f);
        }
        current = rotate_cw(&current);
    }

    result
}

/// Convert cells to be relative to anchor (first cell in raster-sorted order)
fn relative_to_anchor(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let anchor = cells[0];

    cells
        .iter()
        .map(|&(r, c)| (r - anchor.0, c - anchor.1))
        .collect()
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    let mut shapes_raw: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut regions = Vec::new();
    let mut current_shape_lines: Vec<&str> = Vec::new();
    let mut in_shape = false;

    for line in input.lines() {
        if line.contains('x') && line.contains(':') {
            // Region line: "WxH: n0 n1 n2 ..."
            let (dims, counts_str) = line.split_once(':').unwrap();
            let (w, h) = dims.trim().split_once('x').unwrap();
            let w: usize = w.parse().unwrap();
            let h: usize = h.parse().unwrap();
            let counts: Vec<usize> = counts_str
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            regions.push((w, h, counts));
        } else if line.contains(':') && line.trim().ends_with(':') {
            // Shape header: "N:"
            if in_shape && !current_shape_lines.is_empty() {
                shapes_raw.push(parse_shape(&current_shape_lines));
                current_shape_lines.clear();
            }
            in_shape = true;
            current_shape_lines.clear();
        } else if in_shape && !line.is_empty() {
            current_shape_lines.push(line);
        } else if line.is_empty() && in_shape && !current_shape_lines.is_empty() {
            shapes_raw.push(parse_shape(&current_shape_lines));
            current_shape_lines.clear();
            in_shape = false;
        }
    }
    if in_shape && !current_shape_lines.is_empty() {
        shapes_raw.push(parse_shape(&current_shape_lines));
    }

    let shape_cells: Vec<usize> = shapes_raw.iter().map(|s| s.len()).collect();
    let shapes: Vec<Vec<Vec<(i32, i32)>>> = shapes_raw
        .iter()
        .map(|s| {
            all_orientations(s)
                .iter()
                .map(|o| relative_to_anchor(o))
                .collect()
        })
        .collect();

    Input {
        shapes,
        regions,
        shape_cells,
    }
}

fn parse_shape(lines: &[&str]) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                cells.push((r as i32, c as i32));
            }
        }
    }
    
    cells
}

fn can_fit(
    w: usize,
    h: usize,
    shapes: &[Vec<Vec<(i32, i32)>>],
    counts: &[usize],
    shape_cells: &[usize],
) -> bool {
    let area = w * h;
    let total_cells: usize = counts
        .iter()
        .zip(shape_cells.iter())
        .map(|(&c, &s)| c * s)
        .sum();
    if total_cells > area {
        return false;
    }

    let total_shapes: usize = counts.iter().sum();
    if total_shapes == 0 {
        return true;
    }

    // Build list of shape copies to place, ordered by fewest orientations first
    // (most constrained shapes first)
    let mut shape_list: Vec<usize> = Vec::new();
    let mut order: Vec<usize> = (0..counts.len()).collect();
    order.sort_by_key(|&i| shapes[i].len());
    for &i in &order {
        for _ in 0..counts[i] {
            shape_list.push(i);
        }
    }

    let mut grid = vec![0u64; h];
    let skip_budget = area - total_cells;

    solve(&mut grid, w, h, shapes, &shape_list, 0, skip_budget, 0)
}

#[allow(clippy::too_many_arguments)]
fn solve(
    grid: &mut [u64],
    w: usize,
    h: usize,
    shapes: &[Vec<Vec<(i32, i32)>>],
    shape_list: &[usize],
    shape_idx: usize,
    skip_budget: usize,
    search_from: usize,
) -> bool {
    // All shapes placed
    if shape_idx >= shape_list.len() {
        return true;
    }

    // Find first empty cell
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
        return false; // no empty cells but shapes remain
    }
    let r = idx / w;
    let c = idx % w;

    // Remaining cells in grid from this point
    let remaining_cells = w * h - idx;
    let remaining_shape_cells: usize = shape_list[shape_idx..]
        .iter()
        .map(|&s| shapes[s][0].len())
        .sum();
    if remaining_shape_cells > remaining_cells {
        return false;
    }

    // Try placing each remaining shape type at (r, c)
    // To avoid redundant search, for consecutive identical shape types,
    // only try the current one (not later duplicates)
    let current_type = shape_list[shape_idx];

    // Try all orientations of this shape type
    for orient in &shapes[current_type] {
        if try_place(grid, w, h, r, c, orient) {
            place(grid, r, c, orient);

            if solve(
                grid,
                w,
                h,
                shapes,
                shape_list,
                shape_idx + 1,
                skip_budget,
                idx + 1,
            ) {
                return true;
            }
            unplace(grid, r, c, orient);
        }
    }

    // Try skipping this cell (just advance past it)
    if skip_budget > 0
        && solve(
            grid,
            w,
            h,
            shapes,
            shape_list,
            shape_idx,
            skip_budget - 1,
            idx + 1,
        )
    {
        return true;
    }

    false
}

fn try_place(grid: &[u64], w: usize, h: usize, r: usize, c: usize, orient: &[(i32, i32)]) -> bool {
    for &(dr, dc) in orient {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;

        if nr < 0 || nr >= h as i32 || nc < 0 || nc >= w as i32 {
            return false;
        }
        if grid[nr as usize] & (1u64 << nc as usize) != 0 {
            return false;
        }
    }
    true
}

fn place(grid: &mut [u64], r: usize, c: usize, orient: &[(i32, i32)]) {
    for &(dr, dc) in orient {
        let nr = (r as i32 + dr) as usize;
        let nc = (c as i32 + dc) as usize;

        grid[nr] |= 1u64 << nc;
    }
}

fn unplace(grid: &mut [u64], r: usize, c: usize, orient: &[(i32, i32)]) {
    for &(dr, dc) in orient {
        let nr = (r as i32 + dr) as usize;
        let nc = (c as i32 + dc) as usize;

        grid[nr] &= !(1u64 << nc);
    }
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    input
        .regions
        .iter()
        .filter(|(w, h, counts)| can_fit(*w, *h, &input.shapes, counts, &input.shape_cells))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 2);
    }
}
