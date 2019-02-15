use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

trait Environment {
    type Action: Action;
    type State: State;
    fn reset(&mut self) -> ();
    fn current_state(&self) -> Self::State;
    fn current_step(&self) -> usize;
    fn step(&mut self, action: &Self::Action) -> (f64, bool);
    fn all_states(&self) -> HashSet<Self::State>;
    fn all_actions(&self) -> HashSet<Self::Action>;
}
trait State: Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash {}
trait Action: Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash {}
trait Agent {
    type Environment: Environment;
    fn policy(
        &self,
        state: &<Self::Environment as Environment>::State,
        actions: &HashSet<<Self::Environment as Environment>::Action>,
    ) -> <Self::Environment as Environment>::Action;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    row: usize,
    column: usize,
}
impl State for Position {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MoveAction {
    Up,
    Down,
    Left,
    Right,
}
impl Action for MoveAction {}

// 地形効果
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Ordinary,
    Damage__,
    Reward__,
    Block___,
}

/// 地形効果つきの場の平面上の一点
/// (row=0, column=0)
/// +------> Column
/// |
/// |
/// v Row
#[derive(Debug, Clone)]
struct Field {
    grid: Vec<Vec<Cell>>,
    current: Position,
    start: Position,
    max_episode_steps: usize,
    steps: usize,
    slip_rate: f64,
}
impl Field {
    fn new(
        grid: Vec<Vec<Cell>>,
        start: Position,
        max_episode_steps: usize,
        slip_rate: f64,
    ) -> Self {
        Self {
            grid,
            current: start,
            start,
            max_episode_steps,
            steps: 0,
            slip_rate,
        }
    }
    fn current_position(&self) -> Position {
        self.current
    }
    fn current_cell(&self) -> Cell {
        self.grid[self.current.row][self.current.column]
    }
    fn row_size(&self) -> usize {
        self.grid.len()
    }
    fn column_size(&self) -> usize {
        self.grid[0].len()
    }
    fn for_each(&self, mut cb: impl FnMut((usize, usize), Cell) -> ()) -> () {
        self.grid.iter().enumerate().for_each(|(row, columns)| {
            columns.iter().enumerate().for_each(|(column, &cell)| {
                cb((row, column), cell);
            });
        });
    }
}
impl Environment for Field {
    type Action = MoveAction;
    type State = Position;
    fn reset(&mut self) {
        self.current = self.start;
        self.steps = 0;
    }
    fn current_step(&self) -> usize {
        self.steps
    }
    fn current_state(&self) -> Position {
        self.current_position()
    }
    fn step(&mut self, action: &MoveAction) -> (f64, bool) {
        self.steps += 1;

        let slipped_action = if rand::random::<f64>() < self.slip_rate {
            use rand::seq::IteratorRandom;
            self.all_actions()
                .into_iter()
                .choose(&mut rand::thread_rng())
                .unwrap()
        } else {
            *action
        };
        use MoveAction::*;
        let (row, column) = match slipped_action {
            Up => (self.current.row - 1, self.current.column),
            Down => (self.current.row + 1, self.current.column),
            Left => (self.current.row, self.current.column - 1),
            Right => (self.current.row, self.current.column + 1),
        };
        if row < self.row_size() && column < self.column_size() {
            // 進めるかどうか確認
            if self.grid[row][column] != Block___ {
                // 進めた
                self.current.row = row;
                self.current.column = column;
            }
        }
        use Cell::*;
        let force_done = self.steps >= self.max_episode_steps;
        match self.current_cell() {
            // default_reward = -0.04 は歩き回っていると（トータルの）報酬が減っていくことを意味する
            Ordinary => (-0.04, force_done),
            Reward__ => (1.0_f64, true),
            Damage__ => (-1.0_f64, true),
            Block___ => unreachable!(),
        }
    }
    fn all_states(&self) -> HashSet<Self::State> {
        let mut states = HashSet::new();
        self.for_each(|(row, column), cell| {
            match cell {
                // Block___ の中には入れないので
                Cell::Block___ => {}
                _ => {
                    states.insert(Position { row, column });
                }
            }
        });
        states
    }
    fn all_actions(&self) -> HashSet<Self::Action> {
        use MoveAction::*;
        let mut actions = HashSet::new();
        actions.insert(Up);
        actions.insert(Down);
        actions.insert(Left);
        actions.insert(Right);
        actions
    }
}
struct Actor<E: Environment> {
    q: HashMap<<E as Environment>::State, HashMap<<E as Environment>::Action, f64>>,
}
impl<E: Environment> Actor<E> {
    fn softmax(
        x: &HashMap<<E as Environment>::Action, f64>,
    ) -> HashMap<<E as Environment>::Action, f64> {
        let sum = x.iter().fold(0.0_f64, |o, (_, v)| o + v.exp());
        x.into_iter().map(|(k, v)| (*k, v.exp() / sum)).collect()
    }
    fn new() -> Self {
        let q = HashMap::new();
        Self { q }
    }
    fn init(
        &mut self,
        states: &HashSet<<E as Environment>::State>,
        actions: &HashSet<<E as Environment>::Action>,
    ) {
        for state in states {
            let mut o = HashMap::new();
            for action in actions {
                o.insert(*action, rand::random());
            }
            self.q.insert(*state, o);
        }
    }
    fn policy(&mut self, state: &<E as Environment>::State) -> <E as Environment>::Action {
        use rand::distributions::WeightedIndex;
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let actions = self.q.get(&state).unwrap();
        let probs = Self::softmax(actions).into_iter().collect::<Vec<_>>();
        let dist = WeightedIndex::new(probs.iter().map(|(_, v)| v)).unwrap();
        let (action, _) = probs[dist.sample(&mut rng)];
        action
    }
}
struct Critic<E: Environment> {
    v: HashMap<<E as Environment>::State, f64>,
}
impl<E: Environment> Critic<E> {
    fn new() -> Self {
        let v = HashMap::new();
        Self { v }
    }
    fn init(&mut self, states: &HashSet<<E as Environment>::State>) {
        for state in states {
            self.v.insert(*state, 0.0_f64);
        }
    }
}
struct ActorCritic<E: Environment> {
    actor: Actor<E>,
    critic: Critic<E>,
}
impl<E: Environment> ActorCritic<E> {
    fn new() -> Self {
        let actor = Actor::new();
        let critic = Critic::new();
        Self { actor, critic }
    }
    fn train(
        mut self,
        env: &mut E,
        episode_count: usize,
        gamma: f64,
        learning_rate: f64,
        report_interval: usize,
    ) -> f64 {
        let mut total_rewards: Vec<f64> = vec![];
        let mut total_steps: Vec<f64> = vec![];
        let actions = env.all_actions();
        let states = env.all_states();
        self.actor.init(&states, &actions);
        self.critic.init(&states);
        for epi in 0..episode_count {
            env.reset();
            let mut experience = vec![];
            loop {
                let state = env.current_state();
                let action = self.actor.policy(&state);
                let (reward, done) = env.step(&action);
                let n_state = env.current_state();
                experience.push((state, action, n_state, reward));
                let gain = reward + gamma * *self.critic.v.get(&n_state).unwrap();
                let estimated = *self.critic.v.get(&state).unwrap();
                let td = gain - estimated;
                let q = self
                    .actor
                    .q
                    .get_mut(&state)
                    .unwrap()
                    .get_mut(&action)
                    .unwrap();
                *q += learning_rate * td;
                let v = self.critic.v.get_mut(&state).unwrap();
                *v += learning_rate * td;
                if done {
                    break;
                }
            }
            let rewards = experience
                .iter()
                .map(|(_, _, _, reward)| *reward)
                .collect::<Vec<_>>();
            total_steps.push(env.current_step() as f64);
            total_rewards.push(rewards.iter().sum());
            if epi % report_interval == 0 {
                use statrs::statistics::Mean;
                use statrs::statistics::Variance;
                let rewards_mean = total_rewards.mean();
                let rewards_std = total_rewards.std_dev();
                let steps_mean = total_steps.mean();
                let steps_std = total_steps.std_dev();
                dbg!((
                    epi,
                    rewards_mean,
                    rewards_std,
                    steps_mean,
                    steps_std
                ));
            }
        }
        total_rewards.iter().sum()
    }
}

fn main() {
    use Cell::*;
    let slip_rate = 0.0;
    let max_episode_steps = 100;
    let mut field = Field::new(
        vec![
            // vec![Block___, Block___, Block___, Block___, Block___, Block___],
            // vec![Block___, Ordinary, Ordinary, Ordinary, Ordinary, Block___],
            // vec![Block___, Ordinary, Ordinary, Damage__, Ordinary, Block___],
            // vec![Block___, Ordinary, Block___, Ordinary, Ordinary, Block___],
            // vec![Block___, Ordinary, Ordinary, Ordinary, Reward__, Block___],
            // vec![Block___, Block___, Block___, Block___, Block___, Block___],
            vec![Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___],
            vec![Block___, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Block___],
            vec![Block___, Ordinary, Ordinary, Ordinary, Damage__, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Block___],
            vec![Block___, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Block___],
            vec![Block___, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Damage__, Ordinary, Damage__, Ordinary, Ordinary, Block___],
            vec![Block___, Damage__, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Block___],
            vec![Block___, Ordinary, Ordinary, Damage__, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Block___],
            vec![Block___, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Damage__, Ordinary, Ordinary, Block___],
            vec![Block___, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Block___],
            vec![Block___, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Damage__, Ordinary, Ordinary, Reward__, Ordinary, Block___],
            vec![Block___, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Ordinary, Block___],
            vec![Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___, Block___],
        ],
        Position { row: 1, column: 1 },
        max_episode_steps,
        slip_rate,
    );
    let gamma = 0.9_f64;
    let episode_count = 10000000;
    let report_interval = episode_count / 10;
    let learning_rate = 0.1;
    let ac = ActorCritic::<Field>::new();
    let total_reward = ac.train(
        &mut field,
        episode_count,
        gamma,
        learning_rate,
        report_interval,
    );
    dbg!(total_reward);
}
