use std::{
    sync::mpsc::Sender,
    thread::{self, sleep},
    time::Duration,
};

use advent_of_code_2025::day9::{Tile, parse, perimeter};
use foldhash::HashSet;
use macroquad::{Window, prelude::*};
use std::sync::mpsc::{self, Receiver};

const RATE: u64 = 130;

enum Message {
    Size(u64),
    MaxSize(u64, (u64, u64), (u64, u64)),
    BoxChecked,
    Rectangle((u64, u64), (u64, u64)),
    Line((u64, u64), (u64, u64)),
    Finished(u64),
}

enum Draw {
    Size(f32),
    MaxSize(f32, (f32, f32), (f32, f32)),
    BoxChecked,
    Rectangle((f32, f32), (f32, f32)),
    Line((f32, f32), (f32, f32)),
    Finished(f32),
}

impl From<Message> for Draw {
    fn from(value: Message) -> Self {
        match value {
            Message::Rectangle(p1, p2) => Draw::Rectangle(
                ((p1.0 / RATE) as f32, (p1.1 / RATE) as f32),
                ((p2.0 / RATE) as f32, (p2.1 / RATE) as f32),
            ),
            Message::Line(p1, p2) => Draw::Line(
                ((p1.0 / RATE) as f32, (p1.1 / RATE) as f32),
                ((p2.0 / RATE) as f32, (p2.1 / RATE) as f32),
            ),
            Message::Size(v) => Draw::Size(v as f32),
            Message::MaxSize(v, p1, p2) => Draw::MaxSize(
                v as f32,
                ((p1.0 / RATE) as f32, (p1.1 / RATE) as f32),
                ((p2.0 / RATE) as f32, (p2.1 / RATE) as f32),
            ),
            Message::Finished(v) => Draw::Finished(v as f32),
            Message::BoxChecked => Draw::BoxChecked,
        }
    }
}

fn main() {
    let input = include_str!("../../input/2025/day9.txt");

    let tiles = parse(input);
    let red_tiles = tiles
        .iter()
        .map(|t| ((t.x / RATE) as f32, (t.y / RATE) as f32))
        .collect();

    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(|| {
        Window::from_config(
            Conf {
                sample_count: 4,
                window_title: "Day9 - Part2".to_string(),
                window_height: 768,
                window_width: 768,
                fullscreen: false,
                high_dpi: false,
                window_resizable: false,
                ..Default::default()
            },
            process(red_tiles, rx),
        );
        std::process::exit(0);
    });

    let mut max = 0;
    let p = perimeter(&tiles);
    for i in 0..tiles.len() {
        let tile1 = &tiles[i];
        for tile2 in tiles.iter().skip(i + 1) {
            let size = tile2.y.abs_diff(tile1.y + 1) * tile2.x.abs_diff(tile1.x + 1);

            let _ = tx.send(Message::Rectangle((tile1.x, tile1.y), (tile2.x, tile2.y)));
            let _ = tx.send(Message::Size(size));

            if tile1.x != tile2.x
                && tile1.y != tile2.y
                && size > max
                && is_inside(&Tile::new(tile1.x, tile2.y), &p, &tx)
                && is_inside(&Tile::new(tile2.x, tile1.y), &p, &tx)
            {
                let new_perimeter = perimeter(&[
                    tile1.clone(),
                    Tile::new(tile1.x, tile2.y),
                    tile2.clone(),
                    Tile::new(tile2.x, tile1.y),
                ]);

                let mut valid = true;
                for tile in new_perimeter {
                    if !is_inside(&tile, &p, &tx) {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    let _ = tx.send(Message::MaxSize(
                        size,
                        (tile1.x, tile1.y),
                        (tile2.x, tile2.y),
                    ));
                    max = size
                }
            }

            let _ = tx.send(Message::BoxChecked);
        }
    }
    let _ = tx.send(Message::Finished(max));
    loop {
        sleep(Duration::new(3600, 0));
    }
}

fn is_inside(tile: &Tile, perimeter: &HashSet<Tile>, tx: &Sender<Message>) -> bool {
    if perimeter.contains(tile) {
        return true;
    }

    let mut count = 0;
    let mut current = tile.clone();

    while current.x < 99840 {
        if current.x.is_multiple_of(1000) {
            let _ = tx.send(Message::Line(
                (current.x - 1000, current.y),
                (current.x, current.y),
            ));
        }
        if perimeter.contains(&current) {
            count += 1;
            current.x += 1;

            let mut edge = 0;
            while perimeter.contains(&current) {
                current.x += 1;
                edge += 1;
            }

            if edge > 1 {
                count -= 1;
            }
        }

        current.x += 1
    }

    count % 2 == 1
}

async fn process(red_tiles: Vec<(f32, f32)>, rx: Receiver<Message>) {
    let mut messages: Vec<(Draw, f32)> = vec![];
    let mut valid: Vec<(f32, f32)> = vec![];
    let mut current_size_str = "Checking size: 0".to_string();
    let mut max_size_str = "Max found yet: 0".to_string();
    let mut boxes_checked_str = "Boxes checked: 0".to_string();
    let mut finished = false;
    let mut boxes_checked = 0;

    loop {
        if let Ok(m) = rx.recv() {
            messages.push((m.into(), 1.0));
        }

        clear_background(BLACK);

        for pixel in red_tiles.windows(2) {
            draw_line(pixel[0].0, pixel[0].1, pixel[1].0, pixel[1].1, 1.0, RED);
        }

        for vertex in valid.chunks_exact(2) {
            draw_line(
                vertex[0].0,
                vertex[0].1,
                vertex[0].0,
                vertex[1].1,
                1.0,
                GREEN,
            );
            draw_line(
                vertex[0].0,
                vertex[1].1,
                vertex[1].0,
                vertex[1].1,
                1.0,
                GREEN,
            );
            draw_line(
                vertex[1].0,
                vertex[1].1,
                vertex[1].0,
                vertex[0].1,
                1.0,
                GREEN,
            );
            draw_line(
                vertex[1].0,
                vertex[0].1,
                vertex[0].0,
                vertex[0].1,
                1.0,
                GREEN,
            );
        }

        for (v, ttl) in messages.iter_mut() {
            match v {
                Draw::Rectangle(p1, p2) => {
                    let color = Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: *ttl,
                    };
                    draw_line(p1.0, p1.1, p1.0, p2.1, 1.0, color);
                    draw_line(p1.0, p2.1, p2.0, p2.1, 1.0, color);
                    draw_line(p2.0, p2.1, p2.0, p1.1, 1.0, color);
                    draw_line(p2.0, p1.1, p1.0, p1.1, 1.0, color);
                }
                Draw::Line(p1, p2) => {
                    let color = Color {
                        r: 1.0,
                        g: 1.0,
                        b: 0.0,
                        a: *ttl,
                    };
                    draw_line(p1.0, p1.1, p2.0, p2.1, 1.0, color);
                }
                Draw::Size(size) => {
                    current_size_str = format!("Checking size: {size}");
                }
                Draw::Finished(v) => {
                    max_size_str = v.to_string();
                    finished = true;
                }
                Draw::MaxSize(size, p1, p2) => {
                    valid.push(*p1);
                    valid.push(*p2);
                    max_size_str = format!("Max found yet: {size}");
                }
                Draw::BoxChecked => {
                    boxes_checked += 1;
                    boxes_checked_str = format!("Boxes checked: {boxes_checked}");
                }
            }
            *ttl -= 0.005;
        }

        if finished {
            draw_text(&max_size_str, 320.0, 400.0, 150.0, GREEN);
            draw_text(&boxes_checked_str, 320.0, 400.0, 150.0, RED);
        } else {
            draw_text(&current_size_str, 320.0, 200.0, 20.0, DARKGRAY);
            draw_text(&boxes_checked_str, 320.0, 215.0, 20.0, DARKGRAY);
            draw_text(&max_size_str, 320.0, 230.0, 20.0, GREEN);
        }

        messages.retain(|(_, ttl)| *ttl > 0.005);

        next_frame().await
    }
}
