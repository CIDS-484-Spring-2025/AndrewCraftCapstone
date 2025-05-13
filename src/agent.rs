use rand::Rng;
use std::collections::HashMap;
use rand::prelude::IndexedRandom;

type State = (usize, usize); // (x, y)
type Action = usize; // 0 = up, 1 = right, 2 = down, 3 = left
const ACTIONS: [Action; 4] = [0, 1, 2, 3];

#[derive(Debug)]
pub struct Agent {
    pub x: usize,
    pub y: usize,
    pub q_table: HashMap<(State, Action), f32>,
    pub alpha: f32,
    pub gamma: f32,
    pub epsilon: f32,
}

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

impl Agent {
    pub fn new(start_x: usize, start_y: usize) -> Self {
        Self {
            x: start_x,
            y: start_y,
            q_table: HashMap::new(),
            alpha: 0.1,
            gamma: 0.9,
            epsilon: 0.6,
        }
    }

    pub fn reset(&mut self, start_x: usize, start_y: usize) {
        self.x = start_x;
        self.y = start_y;
    }

    pub fn choose_action(&self, state: State) -> Action {
        let mut rng = rand::rng();
        if rng.random::<f32>() < self.epsilon {
            *ACTIONS.choose(&mut rng).unwrap()
        } else {
            let mut best_action = 0;
            let mut best_value = f32::MIN;
            for &action in &ACTIONS {
                let value = *self.q_table.get(&(state, action)).unwrap_or(&0.0);
                if value > best_value {
                    best_value = value;
                    best_action = action;
                }
            }
            best_action
        }
    }

    pub fn take_action(&mut self, action: Action, maze: &[super::Cell], width: usize, height: usize) -> State {
        let current_index = self.y * width + self.x;
        let cell = &maze[current_index];

        let (dx, dy) = match action {
            0 if !cell.walls[0] && self.y > 0 => (0, -1), // Up
            1 if !cell.walls[1] && self.x < width - 1 => (1, 0),  // Right
            2 if !cell.walls[2] && self.y < height - 1 => (0, 1), // Down
            3 if !cell.walls[3] && self.x > 0 => (-1, 0), // Left
            _ => (0, 0), // Invalid move or wall hit
        };

        self.x = (self.x as isize + dx) as usize;
        self.y = (self.y as isize + dy) as usize;
        (self.x, self.y)
    }

    pub fn best_action(&self, state: State) -> Option<Action> {
        let mut best_action = None;
        let mut best_value = f32::MIN;

        for &action in &ACTIONS {
            let value = *self.q_table.get(&(state, action)).unwrap_or(&0.0);
            if value > best_value {
                best_value = value;
                best_action = Some(action);
            }
        }

        best_action
    }

    pub fn next_position(&self, state: State, action: Action) -> State {
        let (x, y) = state;
        match action {
            0 if y > 0 => (x, y - 1),               // Up
            1 => (x + 1, y),                        // Right
            2 => (x, y + 1),                        // Down
            3 if x > 0 => (x - 1, y),               // Left
            _ => (x, y),                            // Invalid / stay
        }
    }

    // This is an admittedly very long function to find which agent had the least steps.
    pub fn get_optimal_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
        maze: &[super::Cell],
        width: usize,
        height: usize,
    ) -> Vec<(usize, usize)> {
        let mut path = vec![start];
        let mut current = start;
        let mut visited = std::collections::HashSet::new();

        while current != goal {
            visited.insert(current);

            if let Some(action) = self.best_action(current) {
                let next = self.next_position(current, action);

                // prevent cycles or getting stuck
                if visited.contains(&next) || next == current {
                    break;
                }

                // validate move against walls
                let idx = current.1 * width + current.0;
                let cell = &maze[idx];
                let wall_blocked = match action {
                    0 => cell.walls[0], // Up
                    1 => cell.walls[1], // Right
                    2 => cell.walls[2], // Down
                    3 => cell.walls[3], // Left
                    _ => true,
                };
                if wall_blocked {
                    break;
                }

                path.push(next);
                current = next;
            } else {
                break;
            }
        }

        path
    }
}


fn update_q_value(
    q_table: &mut HashMap<(State, Action), f32>,
    state: State,
    action: Action,
    reward: f32,
    next_state: State,
    alpha: f32,
    gamma: f32,
) {
    let max_next_q = ACTIONS
        .iter()
        .map(|&a| *q_table.get(&(next_state, a)).unwrap_or(&0.0))
        .fold(f32::MIN, f32::max);

    let current_q = *q_table.get(&(state, action)).unwrap_or(&0.0);
    let new_q = current_q + alpha * (reward + gamma * max_next_q - current_q);
    q_table.insert((state, action), new_q);
}

pub fn train_agent(
    maze: &[super::Cell],
    width: usize,
    height: usize,
    start: State,
    goal: State,
    episodes: usize,
) -> Agent {
    let mut agent = Agent::new(start.0, start.1);

    for episode in 0..episodes {
        agent.reset(start.0, start.1);
        let mut steps = 0;

        while (agent.x, agent.y) != goal && steps < 1000 {
            let state = (agent.x, agent.y);
            let action = agent.choose_action(state);
            let next_state = agent.take_action(action, maze, width, height);

            let reward = if next_state == goal { 100.0 } else { -0.1 };
            update_q_value(
                &mut agent.q_table,
                state,
                action,
                reward,
                next_state,
                agent.alpha,
                agent.gamma,
            );

            steps += 1;
        }

        println!("Episode {episode} completed in {steps} steps");
    }

    println!("Training completed!");
    agent
    
}

pub fn run_trained_agent(
    agent: &mut Agent,
    maze: &[super::Cell],
    width: usize,
    height: usize,
    goal: State,
) -> Vec<State> {
    let mut path = vec![(agent.x, agent.y)];
    let mut steps = 0;

    while (agent.x, agent.y) != goal && steps < 1000 {
        let state = (agent.x, agent.y);
        let action = ACTIONS
            .iter()
            .max_by(|&&a1, &&a2| {
                let q1 = *agent.q_table.get(&(state, a1)).unwrap_or(&0.0);
                let q2 = *agent.q_table.get(&(state, a2)).unwrap_or(&0.0);
                q1.partial_cmp(&q2).unwrap()
            })
            .copied()
            .unwrap_or(0);

        let next_state = agent.take_action(action, maze, width, height);
        path.push(next_state);
        steps += 1;
    }

    path
}


