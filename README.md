# **Maze Solver with Machine Learning in Rust :crab:** 
This project randomly generates solvable mazes, visualizes them as PNG images, and prepares for integration with a machine learning
agent that will learn to navigate through the maze. 

# ******Features******
- Maze generation using randomized depth-search first algorithm
- Start (_S_) and End (_E_) points randomized on the left/right sides
- ASCII and PNG render support (with color-coded start/end)
- Currently now working on the machine learning-driven maze solving (not yet pushable, but getting there!)

# **PNG Output**
Each made is drawn into a PNG using the image crate, where:
- White = Open Paths
- Black = Walls
- Green = Start Point
- Red = End Point

Then, each step of the Machine Learning Agent will be saved as a new frame (frame_000.png, frame_001.png...) into the
frames/ folder
Then can be compiled into a GIF formate using the `gifski` crate with this CLI prompt:

    `gifski --fps 10 -o output.gif frames/frame_*.png`
# **Crates Used**
  - `rand` - for random maze generation
  - `image` - to render PNG files
  - `gifski` - to create GIFs from image frames

# **Running This Code** 
To run, install Rust via [Rustup](https://rustup.rs/), then in Command Line, change your directory to that of the folder containing the src folder and cargo.toml, then run the following two commands:

`cargo build`
</br>`cargo run`
  
