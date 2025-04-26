use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

type State = (usize, usize); // x, y
type Action = usize; // 0 = up, 1 = down, 2 = left, 3 = right
const ACTIONS: [Action; 4] = [0, 1, 2, 3];

// let mut q_table: HashMap<(State, Action), f32> = HashMap::new();

/*
Quick explanation of alpha, gamma, and epsilon here.
Alpha is the learning rate; Determines how much new information overrides old knowledge in the Q-Table.
    so alpha = 1 would mean it always fully trusts new info
     while 0 makes it never update and always sticks to what it knows
Gamma determines how much this agent cares about future rewards vs immediate rewards
    gamma = 0 means it only cares about immediate rewards and gamma = 1 means it values long term rewards equally
Epsilon controls how often our agent takes a random action instead of the one it thinks is best
    epsilon = 0.0 -> never explores; epsilon = 1.0 -> random moves
and we store all of this within a HaseMap
*/

#[derive(Debug)]
pub struct Agent {
    pub x: usize,
    pub y: usize,
    pub q_table: HashMap<(State, Action), f32>,
    pub alpha: f32,
    pub gamma: f32,
    pub epsilon: f32,
}

impl Agent {
    pub fn new(start_x: usize, start_y: usize) -> Self {
        Self {
            x: start_x,
            y: start_y,
            q_table: HashMap::new(),
            alpha: 0.1,
            gamma: 0.9,
            epsilon: 0.2,
        }
    }

    pub fn reset(&mut self, start_x: usize, start_y: usize) {
        self.x = start_x;
        self.y = start_y;
    }

    // TODO add movement, decision making, and other things here
}

pub fn train_agent() {
    println!("Training agent...");
    // TODO: implement Q-learning here
}
