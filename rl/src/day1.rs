#![allow(dead_code)]

// 地形効果
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum Cell { Ordinary, Damage, Reward, Block }

// 地形効果つきの場
// (row=0, column=0)
// +------> Column
// |
// |
// v Row
#[derive(Debug,Clone,PartialEq,Eq)]
struct Field(Vec<Vec<Cell>>);
impl Field {
    fn new(row: usize, column: usize) -> Self {
        let grid = vec![vec![Cell::Ordinary; column]; row];
        Self(grid)
    }
    fn get(&self, row: usize, column: usize) -> Cell {
        self.0[row][column]
    }
    fn row(&self) -> usize {
        self.0.len()
    }
    fn column(&self) -> usize {
        self.0[0].len()
    }
    fn for_each(&self, mut cb: impl FnMut(usize, usize, Cell) -> ()) -> () {
        self.0.iter().enumerate().for_each(|(row, columns)|{
            columns.iter().enumerate().for_each(|(column, &cell)|{
                cb(row, column, cell);
            });
        });
    }
}


/// 平面上の一点
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
struct State {
    row: usize,
    column: usize
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum Action { Up, Down, Left, Right }
impl Action {
    fn opposite(&self) -> Action {
        use Action::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}
#[derive(Debug,Clone,PartialEq)]
struct Environment {
    field: Field,
    agent_state: State,
    default_reward: f64,
    move_prob: f64,
}
impl Environment {
    fn new(field: Field, move_prob: f64) -> Self {
        let agent_state = State{ row: field.row() -1 , column: 0 };
        Self { field, agent_state: agent_state, default_reward: -0.04, move_prob }
    }
    // 左下へ移動
    fn reset(&mut self) -> State {
        self.agent_state = State{ row: self.field.row() -1, column: 0 };
        self.agent_state
    }
    // すべての選択肢
    fn actions(&self) -> Vec<Action> {
        vec![ Action::Up, Action::Down, Action::Left, Action::Right ]
    }
    // すべての可能な状態
    fn states(&self) -> Vec<State> {
        let mut states = vec![];
        self.field.for_each(|row, column, cell|{
            match cell {
                Cell::Block => {},
                _ => {
                    states.push(State{row, column});
                }
            }
        });
        states
    }
    // エージェントが選んだ行動を環境に適用する
    fn step(&mut self, action: Action) -> (State,  f64, bool) {
        // println!("step");
        let (next_state, reward, done) = transit(self, self.agent_state, action);
        self.agent_state = next_state;
        return (next_state, reward, done);
        fn transit(env: &Environment, state: State, action: Action) -> (State, f64, bool) {
            // println!("transit");
            let transition_probs = transit_func(env, state, action);
            let mut choices = vec![];
            for (st, prob) in transition_probs {
                choices.push((st, prob));
            }
            // println!("choices: {:?}", choices);
            use rand::thread_rng;
            use rand::seq::SliceRandom;
            let mut rng = thread_rng();
            let (next_state, _) = *choices.choose_weighted(&mut rng, |item| item.1).unwrap();
            let (reward, done) = reward_func(env, next_state);
            return (next_state, reward, done);
        }
    }
}
use std::collections::HashMap;
fn transit_func(env: &Environment, state: State, action: Action) -> HashMap<State, f64> {
    // println!("transit_func");
    let mut transition_probs = HashMap::new();
    for act in env.actions() {
        // println!("{:?}", act);
        let prob = if act == action { // 行動した方向への遷移確率
            env.move_prob
        } else if act != action.opposite() { // 行動したい方向の左右への遷移確率
            (1_f64 - env.move_prob) / 2_f64
        } else { // 反対側の遷移確率はゼロ
            0_f64
        };
        let next_state = _move(env, state, act);
        // println!("___next_state___: {:?}", next_state);
        if transition_probs.get(&next_state).is_some() {
            *transition_probs.get_mut(&next_state).unwrap() = prob;
        }else {
            transition_probs.insert(next_state, prob);
        }
    }
    return transition_probs;
    // この state は存在可能かどうか
    fn can_action_at(env: &Environment, state: State) -> bool {
        let State { row, column } = state;
        if 0 <= row && row < env.field.row()  && 0 <= column && column < env.field.column() {
            let o = env.field.get(state.row, state.column);
            match o {
                Cell::Ordinary => true,
                _ => false
            }
        }else{ false }
    }
    // 現在の state から action した結果の state へ移動する
    // move できない場合（壁、場外）はその場に留まる
    fn _move(env: &Environment, state: State, action: Action) -> State {
        // println!("move: {:?}, {:?}", state, action);
        if ! can_action_at(env, state) {
            unreachable!("cannot move from here");
        }
        let State { mut row, mut column } = state;
        match action {
            Action::Up =>{
                // prevent 'attempt to subtract with overflow',
                if row == 0 { return state; }
                row -= 1;
            },
            Action::Down => row += 1,
            Action::Left => {
                // prevent 'attempt to subtract with overflow',
                if column == 0 { return state; }
                column -= 1;
            },
            Action::Right => column += 1,
        }
        if 0 <= row && row < env.field.row()  && 0 <= column && column < env.field.column() {
            if env.field.get(row, column) != Cell::Block {
                return State{row, column};
            }
        }
        state
    }
}
fn reward_func(env: &Environment, next_state: State) -> (f64, bool) {
    // println!("reward_func");
    let next_cell = env.field.get(next_state.row, next_state.column);
    match next_cell {
        // default_reward = -0.04 は歩き回っていると（トータルの）報酬が減っていくことを意味する
        Cell::Ordinary => (env.default_reward, false),
        Cell::Reward => (1.0_f64, true),
        Cell::Damage => (-1.0_f64, true),
        Cell::Block => unreachable!()
    }
}

struct Agent{ actions: Vec<Action> }
impl Agent {
    fn new ( actions: Vec<Action> )-> Self { Self { actions } }
    fn policy(&self, state: State) -> Action {
        use rand::thread_rng;
        use rand::seq::SliceRandom;
        let mut rng = thread_rng();
        *self.actions.choose(&mut rng).unwrap()
    }
}

fn main() {
    use Cell::*;
    let grid = Field(vec![
        vec![Ordinary, Ordinary, Ordinary, Reward],
        vec![Ordinary, Ordinary, Damage,   Ordinary],
        vec![Ordinary, Block,    Ordinary, Ordinary],
        vec![Ordinary, Ordinary, Ordinary, Ordinary]
    ]);
    let mut env = Environment::new(grid, 0.08);
    let agent = Agent::new(env.actions());
    for i in 0..100 {
        let mut state = env.reset();
        let mut total_reward = 0.0_f64;
        loop {
            let action = agent.policy(state);
            // println!("Episode {}: current: {:?}, {:?}", i, state, env.field.get(state.row, state.column));
            // println!("Episode {}: Action: {:?}", i, action);
            let (next_state, reward, done) = env.step(action);
            total_reward += reward;
            state = next_state;
            if done { 
                // println!("Episode {}: last: {:?}", i, state);
                break;
            }
        }
        println!("Episode {}: Agent gets {} reward.", i, total_reward);
    }
}
