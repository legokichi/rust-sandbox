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
    fn all_states(&self) -> Vec<Self::State>;
    fn all_actions(&self) -> Vec<Self::Action>;
}
trait State: Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash {
}
trait Action: Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash {
}

trait Agent {
    type Environment: Environment;
    fn policy(&self, state: &<Self::Environment as Environment>::State, actions: &[<Self::Environment as Environment>::Action]) -> <Self::Environment as Environment>::Action;
}
trait QAgent {
    type Environment: Environment;
    fn epsilon(&self) -> f64;
    fn q(&self) -> &HashMap<<Self::Environment as Environment>::State, HashMap<<Self::Environment as Environment>::Action, f64>>;
    fn q_mut(&mut self) -> &mut HashMap<<Self::Environment as Environment>::State, HashMap<<Self::Environment as Environment>::Action, f64>>;
    fn q2_mut(&mut self, state: &<Self::Environment as Environment>::State, action: &<Self::Environment as Environment>::Action) -> &mut f64 {
        if let Some(o) = self.q_mut().get_mut(state) {
            if o.get(action).is_none() {
                o.insert(*action, 0.0);
            }
        }else{
            let mut o = HashMap::new();
            o.insert(*action, 0.0);
            self.q_mut().insert(*state, o);
        }
        self.q_mut().get_mut(&state).unwrap().get_mut(&action).unwrap()
    }
    fn policy(&self, state: &<Self::Environment as Environment>::State, actions: &[<Self::Environment as Environment>::Action]) -> <Self::Environment as Environment>::Action {
        if !(rand::random::<f64>() < self.epsilon()) {
            // 報酬獲得行動
            if let Some(actions_exprement) = self.q().get(&state) {
                if !actions_exprement.is_empty() {
                    // この状況で過去に実行した選択肢の中で一番報酬が多かった選択肢を実行
                    let mut argmax_prob = std::f64::MIN;
                    let mut argmax_action = None;
                    for (action, prob) in actions_exprement.iter() {
                        if *prob >= argmax_prob {
                            argmax_prob = *prob;
                            argmax_action = Some(*action);
                        }
                    }
                    let action = argmax_action.unwrap();
                    // println!("報酬獲得行動: {:?}", some_action);
                    return action;
                }
            }
        }
        // 探索行動
        use rand::seq::SliceRandom;
        // ランダムアクション
        let action = *actions.choose(&mut rand::thread_rng()).unwrap();
        // println!("探索行動: {:?}", some_action);
        action
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    row: usize,
    column: usize,
}
impl State for Position {
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MoveAction {
    Up,
    Down,
    Left,
    Right,
}
impl MoveAction {
    fn opposite(&self) -> MoveAction {
        use MoveAction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}
impl Action for MoveAction {
}

// 地形効果
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Ordinary,
    Damage,
    Reward,
    Block,
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
    default_reward: f64,
    max_episode_steps: usize,
    step_count: usize,
}
impl Field {
    fn new(grid: Vec<Vec<Cell>>, start: Position, max_episode_steps: usize) -> Self {
        Self {
            grid,
            current: start,
            start,
            default_reward: -0.04,
            max_episode_steps,
            step_count: 0,
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
        self.step_count = 0;
    }
    fn current_step(&self) -> usize {
        self.step_count
    }
    fn step(&mut self, action: &MoveAction) -> (f64, bool) {
        self.step_count += 1;
        use MoveAction::*;
        let (row, column) = match *action {
            Up => (self.current.row - 1, self.current.column),
            Down => (self.current.row + 1, self.current.column),
            Left => (self.current.row, self.current.column - 1),
            Right => (self.current.row, self.current.column + 1),
        };
        if row < self.row_size() && column < self.column_size() {
            // 進めるかどうか確認
            if self.grid[row][column] != Block {
                // 進めた
                self.current.row = row;
                self.current.column = column;
            }
        }
        use Cell::*;
        match self.current_cell() {
            // default_reward = -0.04 は歩き回っていると（トータルの）報酬が減っていくことを意味する
            Ordinary => (self.default_reward, self.step_count >= self.max_episode_steps),
            Reward => (1.0_f64, true),
            Damage => (-1.0_f64, true),
            Block => unreachable!(),
        }
    }
    fn current_state(&self) -> Position {
        self.current_position()
    }
    fn all_states(&self) -> Vec<Self::State> {
        let mut states = vec![];
        self.for_each(|(row, column), cell|{
            match cell {
                // Block の中には入れないので
                Cell::Block => {},
                _ => {
                    states.push(Position{row, column});
                }
            }
        });
        states
    }
    fn all_actions(&self) -> Vec<Self::Action> {
        use MoveAction::*;
        vec![Up, Down, Left, Right]
    }
}

struct MonteCarloAgent<E: Environment> {
    epsilon: f64,
    q: HashMap<<E as Environment>::State, HashMap<<E as Environment>::Action, f64>>,
}
impl<E: Environment> MonteCarloAgent<E> {
    fn new(epsilon: f64) -> Self {
        let q = HashMap::new();
        Self { epsilon, q }
    }
    fn learn(&mut self, env: &mut E, episode_count: usize, gamma: f64, report_interval: usize) -> f64
    where
        Self: QAgent<Environment = E>,
    {
        let mut total_rewards: Vec<f64> = vec![];
        let mut N: HashMap<<E as Environment>::State, HashMap<<E as Environment>::Action, f64>> = HashMap::new();
        for epi in 0..episode_count {
            env.reset();
            let mut experience = vec![];
            // 試行フェーズ
            loop {
                let state = env.current_state();
                let action = <Self as QAgent>::policy(self, &state, &env.all_actions());
                let (reward, done) = env.step(&action);
                experience.push((state, action, reward));
                if done { break; }
            }
            // この episode における結果をポリシーに反映する
            for (i, exp) in experience.iter().enumerate() {
                let (state, action, reward) = exp;
                let mut G = 0.0_f64;
                let mut t = 0.0_f64;
                for j in i..experience.len() {
                    G += gamma.powf(t) * reward;
                    t += 1.0;
                }
                if let Some(o) = N.get_mut(&state) {
                    if o.get(&action).is_none() {
                        o.insert(*action, 0.0);
                    }
                }else{
                    let mut o = HashMap::new();
                    o.insert(*action, 0.0);
                    N.insert(*state, o);
                }
                let tmp_n = N.get_mut(&state).unwrap().get_mut(&action).unwrap();
                *tmp_n += 1.0;
                let alpha = 1.0 / *tmp_n;
                let tmp_q = self.q2_mut(state, action);
                *tmp_q += alpha * (G - *tmp_q);
            }
            // 結果ログ
            let mut rewards = experience.iter().map(|(_, _, reward)| *reward).collect::<Vec<_>>();
            if epi % report_interval == 0 {
                use statrs::statistics::Variance;
                use statrs::statistics::Mean;
                let mean = rewards.mean();
                let std = rewards.std_dev();
                println!("episode: {}, step: {}, mean: {}, std: {}", epi, env.current_step(), mean, std);
            }
            total_rewards.append(&mut rewards);
        }
        // println!("Q: {:?}", self.q());
        total_rewards.iter().sum()
    }
}
impl Agent for MonteCarloAgent<Field> {
    type Environment = Field;
    fn policy(&self, state: &<Self::Environment as Environment>::State, actions: &[<Self::Environment as Environment>::Action]) -> <Self::Environment as Environment>::Action {
        <Self as QAgent>::policy(self, state, actions)
    }
}
impl QAgent for MonteCarloAgent<Field> {
    type Environment = Field;
    fn epsilon(&self) -> f64 {
        self.epsilon
    }
    fn q(&self) -> &HashMap<<Self::Environment as Environment>::State, HashMap<<Self::Environment as Environment>::Action, f64>> {
        &self.q
    }
    fn q_mut(&mut self) -> &mut HashMap<<Self::Environment as Environment>::State, HashMap<<Self::Environment as Environment>::Action, f64>> {
        &mut self.q
    }
}

struct QLearningAgent<E: Environment> {
    epsilon: f64,
    q: HashMap<<E as Environment>::State, HashMap<<E as Environment>::Action, f64>>,
}
impl<E: Environment> QLearningAgent<E> {
    fn new(epsilon: f64) -> Self {
        let q = HashMap::new();
        Self { epsilon, q }
    }
    fn learn(&mut self, env: &mut E, episode_count: usize, gamma: f64, learning_rate: f64, report_interval: usize) -> f64
    where
        Self: QAgent<Environment = E>,
    {
        let mut total_rewards: Vec<f64> = vec![];
        let mut N: HashMap<<E as Environment>::State, HashMap<<E as Environment>::Action, f64>> = HashMap::new();
        for epi in 0..episode_count {
            env.reset();
            let mut experience = vec![];
            loop {
                let state = env.current_state();
                let action = <Self as QAgent>::policy(self, &state, &env.all_actions());
                let (reward, done) = env.step(&action);
                let n_state = env.current_state();
                experience.push((state, action, reward));
                // TD(1) は 都度推測する
                let max = self.q().get(&n_state).unwrap_or(&HashMap::new()).iter().fold(0.0_f64, |o, (k,v)| o.max(*v));
                let gain = reward + gamma * max;
                let estimated = *self.q2_mut(&state, &action);
                *self.q2_mut(&state, &action) += learning_rate * (gain - estimated);
                if done { break; }
            }
            let mut rewards = experience.iter().map(|(_, _, reward)| *reward).collect::<Vec<_>>();
            if epi % report_interval == 0 {
                use statrs::statistics::Variance;
                use statrs::statistics::Mean;
                let mean = rewards.mean();
                let std = rewards.std_dev();
                println!("episode: {}, step: {}, mean: {}, std: {}", epi, env.current_step(), mean, std);
            }
            total_rewards.append(&mut rewards);
        }
        // println!("Q: {:?}", self.q());
        total_rewards.iter().sum()
    }
}
impl Agent for QLearningAgent<Field> {
    type Environment = Field;
    fn policy(&self, state: &<Self::Environment as Environment>::State, actions: &[<Self::Environment as Environment>::Action]) -> <Self::Environment as Environment>::Action {
        <Self as QAgent>::policy(self, state, actions)
    }
}
impl QAgent for QLearningAgent<Field> {
    type Environment = Field;
    fn epsilon(&self) -> f64 {
        self.epsilon
    }
    fn q(&self) -> &HashMap<<Self::Environment as Environment>::State, HashMap<<Self::Environment as Environment>::Action, f64>> {
        &self.q
    }
    fn q_mut(&mut self) -> &mut HashMap<<Self::Environment as Environment>::State, HashMap<<Self::Environment as Environment>::Action, f64>> {
        &mut self.q
    }
}



struct SARSAAgent<E: Environment> {
    epsilon: f64,
    q: HashMap<<E as Environment>::State, HashMap<<E as Environment>::Action, f64>>,
}
impl<E: Environment> SARSAAgent<E> {
    fn new(epsilon: f64) -> Self {
        let q = HashMap::new();
        Self { epsilon, q }
    }
    fn learn(&mut self, env: &mut E, episode_count: usize, gamma: f64, learning_rate: f64, report_interval: usize) -> f64
    where
        Self: QAgent<Environment = E>,
    {
        let mut total_rewards: Vec<f64> = vec![];
        let mut N: HashMap<<E as Environment>::State, HashMap<<E as Environment>::Action, f64>> = HashMap::new();
        for epi in 0..episode_count {
            env.reset();
            let mut experience = vec![];
            loop {
                let state = env.current_state();
                let action = <Self as QAgent>::policy(self, &state, &env.all_actions());
                let (reward, done) = env.step(&action);
                experience.push((state, action, reward));
                // SARSA はいままでの経験から次の行動を一旦決めてから、その行動の結果を評価する
                let n_state = env.current_state();
                let n_action = <Self as QAgent>::policy(self, &n_state, &env.all_actions()); // On-policy
                let gain = reward + gamma * *self.q2_mut(&n_state, &n_action);
                let estimated = *self.q2_mut(&state, &action);
                *self.q2_mut(&state, &action) += learning_rate * (gain - estimated);
                if done { break; }
            }
            let mut rewards = experience.iter().map(|(_, _, reward)| *reward).collect::<Vec<_>>();
            if epi % report_interval == 0 {
                use statrs::statistics::Variance;
                use statrs::statistics::Mean;
                let mean = rewards.mean();
                let std = rewards.std_dev();
                println!("episode: {}, step: {}, mean: {}, std: {}", epi, env.current_step(), mean, std);
            }
            total_rewards.append(&mut rewards);
        }
        // println!("Q: {:?}", self.q());
        total_rewards.iter().sum()
    }
}
impl Agent for SARSAAgent<Field> {
    type Environment = Field;
    fn policy(&self, state: &<Self::Environment as Environment>::State, actions: &[<Self::Environment as Environment>::Action]) -> <Self::Environment as Environment>::Action {
        <Self as QAgent>::policy(self, state, actions)
    }

}
impl QAgent for SARSAAgent<Field> {
    type Environment = Field;
    fn epsilon(&self) -> f64 {
        self.epsilon
    }
    fn q(&self) -> &HashMap<<Self::Environment as Environment>::State, HashMap<<Self::Environment as Environment>::Action, f64>> {
        &self.q
    }
    fn q_mut(&mut self) -> &mut HashMap<<Self::Environment as Environment>::State, HashMap<<Self::Environment as Environment>::Action, f64>> {
        &mut self.q
    }
}

struct Actor<E: Environment> {
    q: HashMap<<E as Environment>::State, HashMap<<E as Environment>::Action, f64>>,
}
impl<E: Environment> Actor<E> {
    fn softmax(xs: &[f64]) -> Vec<f64> {
        let sum = xs.iter().fold(0.0_f64, |o, p| o + p.exp());
        xs.iter().map(|x| x.exp() / sum).collect()
    }
    fn policy(&mut self, state: &<E as Environment>::State, actions: &HashSet<<E as Environment>::Action>) -> <E as Environment>::Action {
        use rand::prelude::*;
        use rand::distributions::WeightedIndex;
        let mut rng = thread_rng();
        if !self.q.contains_key(&state) {
            let mut o = HashMap::new();
            for act in actions {
                o.insert(*act, rand::random());
            }
            self.q.insert(*state, o);
        }
        let actions = self.q.get(&state).unwrap().iter().collect::<Vec<_>>();
        let values = actions.iter().map(|(_, val)| **val).collect::<Vec<_>>();
        let actions = actions.iter().map(|(act, _)| **act).collect::<Vec<_>>();
        let probs = Self::softmax(&values);
        let dist = WeightedIndex::new(&probs).unwrap();
        let action = actions[dist.sample(&mut rng)];
        action
    }
}
struct Critic<E: Environment> {
    v: HashMap<<E as Environment>::State, f64>,
}
struct ActorCritic<E: Environment>{
    actor: Actor<E>,
    critic: Critic<E>,
}
impl<E: Environment> ActorCritic<E> {
    fn new() -> Self {
        let actor = Actor{ q: HashMap::new() };
        let critic =  Critic{ v: HashMap::new() };
        Self { actor, critic }
    }
    fn train(mut self, env: &mut E, episode_count: usize, gamma: f64, learning_rate: f64, report_interval: usize) -> f64 {
        let mut total_rewards: Vec<f64> = vec![];
        let actions = env.all_actions().into_iter().collect::<HashSet<_>>();
        for epi in 0..episode_count {
            env.reset();
            let mut experience = vec![];
            loop {
                let state = env.current_state();
                let action = self.actor.policy(&state, &actions);
                let (reward, done) = env.step(&action);
                experience.push((state, action, reward));
                let n_state = env.current_state();
                if !self.critic.v.contains_key(&state) {
                    self.critic.v.insert(state, 0.0_f64);
                }
                if !self.critic.v.contains_key(&n_state) {
                    self.critic.v.insert(n_state, 0.0_f64);
                }
                let gain = reward + gamma * *self.critic.v.get(&n_state).unwrap();
                let estimated = *self.critic.v.get(&state).unwrap();
                let td = gain - estimated;
                *self.actor.q.get_mut(&state).unwrap().get_mut(&action).unwrap() += learning_rate * td;
                *self.critic.v.get_mut(&state).unwrap() += learning_rate * td;
                if done { break; }
            }
            let mut rewards = experience.iter().map(|(_, _, reward)| *reward).collect::<Vec<_>>();
            if epi % report_interval == 0 {
                use statrs::statistics::Variance;
                use statrs::statistics::Mean;
                let mean = rewards.mean();
                let std = rewards.std_dev();
                println!("episode: {}, step: {}, mean: {}, std: {}", epi, env.current_step(), mean, std);
            }
            total_rewards.append(&mut rewards);
        }
        total_rewards.iter().sum()
    }
}





fn main() {
    use Cell::*;
    let max_episode_steps = 100;
    let mut field = Field::new(
        vec![
            vec![Block, Block,    Block,    Block,    Block,    Block],
            vec![Block, Ordinary, Ordinary, Ordinary, Ordinary, Block],
            vec![Block, Ordinary, Ordinary, Damage,   Ordinary, Block],
            vec![Block, Ordinary, Block,    Ordinary, Ordinary, Block],
            vec![Block, Ordinary, Ordinary, Ordinary, Reward,   Block],
            vec![Block, Block,    Block,    Block,    Block,    Block],
        ],
        Position{row: 1, column: 1 },
        max_episode_steps
    );
    println!("スタート地点: {:?}, {:?}", field.current_position(), field.current_cell());
    let gamma = 0.9_f64;
    let episode_count = 1000000;
    let report_interval = episode_count/10;
    let learning_rate = 0.1;
    for epsilon in vec![/*0.0,*/ 0.1, /*1.0*/] {
        println!("!!!!BEGIN, MonteCarloAgent, epsilon: {} !!!!", epsilon);
        let mut agent = MonteCarloAgent::<Field>::new(epsilon);
        let total_reward = agent.learn(&mut field, episode_count, gamma, report_interval);
        println!("total_reward: {}", total_reward);
        println!("!!!!END, MonteCarloAgent, epsilon: {} !!!!", epsilon);
    }
    for epsilon in vec![/*0.0,*/ 0.1, /*1.0*/] {
        println!("!!!!BEGIN, QLearningAgent, epsilon: {} !!!!", epsilon);
        let mut qagent = QLearningAgent::<Field>::new(epsilon);
        let total_reward = qagent.learn(&mut field, episode_count, gamma, learning_rate , report_interval);
        println!("total_reward: {}", total_reward);
        println!("!!!!END, QLearningAgent, epsilon: {} !!!!", epsilon);
    }
    for epsilon in vec![/*0.0,*/ 0.1, /*1.0*/] {
        println!("!!!!BEGIN, SARSAAgent, epsilon: {} !!!!", epsilon);
        let mut sarsa = SARSAAgent::<Field>::new(epsilon);
        let total_reward = sarsa.learn(&mut field, episode_count, gamma, learning_rate, report_interval);
        println!("total_reward: {}", total_reward);
        println!("!!!!END, SARSAAgent, epsilon: {} !!!!", epsilon);
    }
    for epsilon in vec![/*0.0,*/ 0.1, /*1.0*/] {
        println!("!!!!BEGIN, ActorCritic, epsilon: {} !!!!", epsilon);
        let mut ac = ActorCritic::<Field>::new();
        let total_reward = ac.train(&mut field, episode_count, gamma, learning_rate, report_interval);
        println!("total_reward: {}", total_reward);
        println!("!!!!END, ActorCritic, epsilon: {} !!!!", epsilon);
    }
}
