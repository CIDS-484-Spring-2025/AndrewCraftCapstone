// Andrew Craft
// CIDS 484-01

mod agent;
use agent::{train_agent, Agent};
use std::process::Command;
mod render;
use render::draw_maze_to_png;

use rand::rng;
use rand::prelude::SliceRandom;
use rand::Rng;
const WIDTH: usize = 20;
const HEIGHT: usize = 20;

static mut START_POS: (usize, usize) = (0, 0);
static mut END_POS: (usize, usize) = (0, 0);

#[derive(Clone, Copy)]
struct Cell {
    visited: bool,
    walls: [bool; 4], // top, right, bottom, left
}

impl Cell {
    fn new() -> Self {
        Cell {
            visited: false,
            walls: [true; 4],
        }
    }
}

fn index(x: isize, y: isize) -> Option<usize> {
    if x < 0 || y < 0 || x >= WIDTH as isize || y >= HEIGHT as isize {
        None
    } else {
        Some((y as usize) * WIDTH + (x as usize))
    }
}

fn generate_maze(grid: &mut Vec<Cell>, x: isize, y: isize) {
    let directions = [(0, -1, 0, 2), (1, 0, 1, 3), (0, 1, 2, 0), (-1, 0, 3, 1)]; 
    let mut rng = rng();
    let mut shuffled = directions.to_vec();
    shuffled.shuffle(&mut rng);

    let current_idx = index(x, y).unwrap();
    grid[current_idx].visited = true;

    for (dx, dy, wall, opposite_wall) in shuffled {
        let nx = x + dx;
        let ny = y + dy;

        if let Some(n_idx) = index(nx, ny) {
            if !grid[n_idx].visited {
                grid[current_idx].walls[wall] = false;
                grid[n_idx].walls[opposite_wall] = false;
                generate_maze(grid, nx, ny);
            }
        }
    }
}

fn set_start_and_end() {
    let mut rng = rand::rng();

    let sy = rng.random_range(0..HEIGHT);
    let ey = rng.random_range(0..HEIGHT);

    unsafe {
        START_POS = (0, sy);
        END_POS = (WIDTH - 1, ey);
    }
}

fn print_maze(grid: &Vec<Cell>) {
    // Top border
    print!(" ");
    for _ in 0..WIDTH {
        print!("___");
    }
    println!();

    for y in 0..HEIGHT {
        print!("|");
        for x in 0..WIDTH {
            let idx = index(x as isize, y as isize).unwrap();
            let cell = grid[idx];

            // Bottom wall
            if cell.walls[2] {
                print!("_");
            } else {
                print!(" ");
            }

            // Content: S, E
            let mut symbol = " ";
            unsafe {
                if (x, y) == START_POS {
                    symbol = "S";
                } else if (x, y) == END_POS {
                    symbol = "E";
                }
            }
            print!("{}", symbol);

            // Right wall
            if cell.walls[1] {
                print!("|");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let mut grid = vec![Cell::new(); WIDTH * HEIGHT];
    generate_maze(&mut grid, 0, 0);
    set_start_and_end(); // pick random S and E
    print_maze(&grid);

    draw_maze_to_png(&grid, 0, None); // Starts at zero for numbering each subsequent PNG for ease of use with gifski
    println!("Maze image saved to maze.png");
    
    let trained_agent;
    let path;
    
    unsafe {
        train_agent(
            &grid,
            WIDTH,
            HEIGHT,
            START_POS,
            END_POS,
            2000);

        trained_agent = Agent::new(START_POS.0, START_POS.1);

        
        path = trained_agent.get_optimal_path(
            START_POS,
            END_POS,
            &grid,
            WIDTH,
            HEIGHT,
        );
    }
    



    // Section of doing the drawing
    let _ = std::fs::remove_dir_all("frames");
    let _ = std::fs::create_dir("frames");
    
    // Draw each step in the path as a frame
    for (i, _step) in path.iter().enumerate() {
        let partial_path = &path[..=i]; // cumulative path up to this step
        draw_maze_to_png(&grid, i + 1, Some(partial_path));
    }

    // Combine PNGs into GIF using gifski. I don't entirely comprehend what is going on here :)
    let status = Command::new("gifski")
        .args([
            "-o", "agent_path.gif",
            "-r", "10", // 10 FPS
        ])
        .arg("frames/frame_*.png")
        .status()
        .expect("Failed to run gifski");

    if status.success() {
        println!("GIF successfully created: agent_path.gif");
    } else {
        println!("GIF creation failed.");
    }
}
