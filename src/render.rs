/// Andrew Craft
/// CIDS 484-01

///This file draws the maze data (not the ASCII art) generated from main.rs into an image with the help of the image crate
use image::{Rgb, RgbImage};
use std::fs;
use crate::{Cell, WIDTH, HEIGHT, index, START_POS, END_POS};

const CELL_SIZE: u32 = 20;
const WALL_THICKNESS: u32 = 2;

/// Public function to be called from main, automatically names frame with zero-padded number.
pub fn draw_maze_to_png(grid: &Vec<Cell>, frame_num: usize) {
    // Make sure frames directory exists
    fs::create_dir_all("frames").expect("Failed to create frames directory");

    // Format the filename with zero-padded frame number
    let filename = format!("frames/frame_{:03}.png", frame_num);
    draw_maze_to_png_with_filename(grid, &filename);
}

/// Internal function that does the actual drawing and saving
fn draw_maze_to_png_with_filename(grid: &Vec<Cell>, filename: &str) {
    let img_width = WIDTH as u32 * CELL_SIZE + WALL_THICKNESS;
    let img_height = HEIGHT as u32 * CELL_SIZE + WALL_THICKNESS;
    let mut img = RgbImage::new(img_width, img_height);

    let white = Rgb([255, 255, 255]);
    let black = Rgb([0, 0, 0]);
    let red = Rgb([255, 0, 0]);
    let green = Rgb([0, 255, 0]);

    // Fill background white
    for pixel in img.pixels_mut() {
        *pixel = white;
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = index(x as isize, y as isize).unwrap();
            let cell = grid[idx];

            let x0 = x as u32 * CELL_SIZE;
            let y0 = y as u32 * CELL_SIZE;
            let x1 = x0 + CELL_SIZE;
            let y1 = y0 + CELL_SIZE;

            // Top wall
            if cell.walls[0] {
                for dx in 0..CELL_SIZE {
                    for t in 0..WALL_THICKNESS {
                        img.put_pixel(x0 + dx, y0 + t, black);
                    }
                }
            }
            // Right wall
            if cell.walls[1] {
                for dy in 0..CELL_SIZE {
                    for t in 0..WALL_THICKNESS {
                        img.put_pixel(x1 - 1 + t, y0 + dy, black);
                    }
                }
            }
            // Bottom wall
            if cell.walls[2] {
                for dx in 0..CELL_SIZE {
                    for t in 0..WALL_THICKNESS {
                        img.put_pixel(x0 + dx, y1 - 1 + t, black);
                    }
                }
            }
            // Left wall
            if cell.walls[3] {
                for dy in 0..CELL_SIZE {
                    for t in 0..WALL_THICKNESS {
                        img.put_pixel(x0 + t, y0 + dy, black);
                    }
                }
            }
        }
    }

    // Draw start (green) and end (red)
    unsafe {
        let (sx, sy) = START_POS;
        let (ex, ey) = END_POS;

        for dy in 0..CELL_SIZE {
            for dx in 0..CELL_SIZE {
                img.put_pixel(sx as u32 * CELL_SIZE + dx, sy as u32 * CELL_SIZE + dy, green);
                img.put_pixel(ex as u32 * CELL_SIZE + dx, ey as u32 * CELL_SIZE + dy, red);
            }
        }
    }

    img.save(filename).expect("Failed to save image");
}
