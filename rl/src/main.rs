mod prelude {
    pub use std::collections::{HashMap, HashSet};
    pub use std::fmt::Debug;
    pub use std::hash::Hash;
    pub trait State: Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash {}
    pub trait Action: Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash {}
    pub trait Environment {
        type Action: Action;
        type State: State;
        fn reset(&mut self) -> ();
        fn current_state(&self) -> Self::State;
        fn current_step(&self) -> usize;
        fn step(&mut self, action: Self::Action) -> (f64, bool);
        fn action_space(&self) -> HashSet<Self::Action>;
        fn state_space(&self) -> HashSet<Self::State>;
    }
    pub trait Agent {
        type Environment: Environment;
        fn new<T>(epsilon: f64, env: &Self::Environment, options: T) -> Self;
        fn initialize(&mut self, experiences: ());
        fn estimate(
            &self,
            state: &<Self::Environment as Environment>::State,
        ) -> HashMap<<Self::Environment as Environment>::Action, f64>;
        fn argmax(
            estimates: &HashMap<<Self::Environment as Environment>::Action, f64>,
        ) -> (<Self::Environment as Environment>::Action, f64) {
            use rand::distributions::Distribution; ;
            use rand::distributions::WeightedIndex;
            let mut rng = rand::thread_rng();
            let probs = estimates.iter().collect::<Vec<_>>();
            let dist = WeightedIndex::new(probs.iter().map(|(_, v)| *v)).unwrap();
            let (action, value) = probs[dist.sample(&mut rng)];
            (*action, *value)
        }
        fn update(&mut self, experiences: (), gamma: f64);
        fn epsilon(&self) -> f64;
        fn actions(&self) -> &[<Self::Environment as Environment>::Action];
        fn policy(
            &self,
            state: &<Self::Environment as Environment>::State,
        ) -> (
            <Self::Environment as Environment>::Action,
            Option<HashMap<<Self::Environment as Environment>::Action, f64>>,
        ) {
            if !(rand::random::<f64>() < self.epsilon()) {
                // 報酬獲得行動
                let action_probs = self.estimate(state);
                let (action, _) = Self::argmax(&action_probs);
                return (action, Some(action_probs));
            }
            // 探索行動
            use rand::seq::SliceRandom;
            // ランダムアクション
            let action = *self.actions().choose(&mut rand::thread_rng()).unwrap();
            // println!("探索行動: {:?}", some_action);
            (action, None)
        }
    }
    pub struct Experience<T: Trainer> {
        pub episode: usize,
        pub prev_step: usize,
        pub prev_state: <<T as Trainer>::Environment as Environment>::State,
        pub dist: Option<HashMap<<<T as Trainer>::Environment as Environment>::Action, f64>>,
        pub action: <<T as Trainer>::Environment as Environment>::Action,
        pub next_step: usize,
        pub next_state: <<T as Trainer>::Environment as Environment>::State,
        pub reward: f64,
    }
    pub trait Trainer: Sized {
        type Environment: Environment;
        type Agent: Agent<Environment = Self::Environment>;
        fn agent(&self) -> &Self::Agent;
        fn play(&mut self, env: &mut Self::Environment, episode_count: usize) {
            let mut experiences = vec![];
            for episode in 0..episode_count {
                env.reset();
                self.on_episode_start();
                let mut experience = vec![];
                loop {
                    let prev_step = env.current_step();
                    let prev_state = env.current_state();
                    self.on_before_step(episode, prev_step, &prev_state);
                    let (action, dist) = self.agent().policy(&prev_state);
                    let (reward, done) = env.step(action);
                    let next_step = env.current_step();
                    let next_state = env.current_state();
                    let expr = Experience {
                        episode,
                        prev_step,
                        prev_state,
                        dist,
                        action,
                        next_step,
                        next_state,
                        reward,
                    };
                    self.on_after_step(&expr);
                    experience.push(expr);
                    // ここでトレーニング

                    if done {
                        break;
                    }
                }
                self.on_episode_end(&experience);
                experiences.push(experience);
            }
        }
        fn on_episode_start(&mut self);
        fn on_episode_end(&mut self, experiences: &[Experience<Self>]);
        fn on_before_step(
            &mut self,
            episode: usize,
            prev_step: usize,
            prev_state: &<Self::Environment as Environment>::State,
        );
        fn on_after_step(&mut self, experiences: &Experience<Self>);
    }
}
mod frozen_lake {
    use super::prelude::*;
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Position {
        pub row: usize,
        pub column: usize,
    }
    impl State for Position {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Move {
        Up,
        Down,
        Left,
        Right,
    }
    impl Move {
        fn opposite(&self) -> Move {
            use Move::*;
            match self {
                Up => Down,
                Down => Up,
                Left => Right,
                Right => Left,
            }
        }
    }
    impl Action for Move {}

    // 地形効果
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Cell {
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
    pub struct Field {
        pub grid: Vec<Vec<Cell>>,
        pub current: Position,
        pub start: Position,
        pub default_reward: f64,
        pub max_episode_steps: usize,
        pub step_count: usize,
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
        type Action = Move;
        type State = Position;
        fn reset(&mut self) {
            self.current = self.start;
            self.step_count = 0;
        }
        fn current_step(&self) -> usize {
            self.step_count
        }
        fn step(&mut self, action: Move) -> (f64, bool) {
            self.step_count += 1;
            use Move::*;
            let (row, column) = match action {
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
                Ordinary => (
                    self.default_reward,
                    self.step_count >= self.max_episode_steps,
                ),
                Reward => (1.0_f64, true),
                Damage => (-1.0_f64, true),
                Block => unreachable!(),
            }
        }
        fn current_state(&self) -> Position {
            self.current_position()
        }
        fn action_space(&self) -> HashSet<Self::Action> {
            use Move::*;
            vec![Up, Down, Left, Right].into_iter().collect()
        }
        fn state_space(&self) -> HashSet<Self::State> {
            let mut states = HashSet::new();
            self.for_each(|(row, column), cell| {
                match cell {
                    // Block の中には入れないので
                    Cell::Block => {}
                    _ => {
                        states.insert(Position { row, column });
                    }
                }
            });
            states
        }
    }
}
mod a2c {
    use super::frozen_lake;
    use super::prelude::*;
    use rayon::prelude::*;
    pub struct A2CAgent {}
    impl A2CAgent {}
    impl Agent for A2CAgent {
        type Environment = frozen_lake::Field;
        fn new<T>(epsilon: f64, env: &Self::Environment, options: T) -> Self {
            unimplemented!()
        }
        fn initialize(&mut self, experiences: ()) {
            unimplemented!()
            // use coaster::prelude::*;
            // use juice::layer::*;
            // use juice::layers::*;
            // use std::rc::Rc;
            // use std::sync::{Arc, RwLock};

            // let backend =  Rc::new(juice::util::native_backend());
            // let mut cfg = SequentialConfig::default();
            // let mut network = Layer::from_config(
            //     backend.clone(),
            //     &LayerConfig::new("foo", LayerType::Sequential(cfg))
            // );

            // let inp = SharedTensor::new(&[128, 3, 231, 231]);
            // let inp_lock = Arc::new(RwLock::new(inp));
            // let o = network.forward(&[inp_lock.clone()]);

            // let mut solver_cfg = SolverConfig { minibatch_size: batch_size, base_lr: learning_rate, momentum: momentum, .. SolverConfig::default() };
            // solver_cfg.network = LayerConfig::new("network", net_cfg);
            // solver_cfg.objective = LayerConfig::new("classifier", classifier_cfg);
            // let mut solver = Solver::from_config(backend.clone(), backend.clone(), &solver_cfg);
        }
        fn estimate(
            &self,
            state: &<Self::Environment as Environment>::State,
        ) -> HashMap<<Self::Environment as Environment>::Action, f64> {
            unimplemented!()
        }
        fn update(&mut self, experiences: (), gamma: f64) {
            unimplemented!()
        }
        fn epsilon(&self) -> f64 {
            unimplemented!()
        }
        fn actions(&self) -> &[<Self::Environment as Environment>::Action] {
            unimplemented!()
        }
    }
    pub struct A2CTrainer {
        agent: A2CAgent,
    }
    impl Trainer for A2CTrainer {
        type Environment = frozen_lake::Field;
        type Agent = A2CAgent;
        fn agent(&self) -> &Self::Agent {
            &self.agent
        }
        fn on_episode_start(&mut self) {}
        fn on_episode_end(&mut self, experiences: &[Experience<Self>]) {}
        fn on_before_step(
            &mut self,
            episode: usize,
            prev_step: usize,
            prev_state: &<Self::Environment as Environment>::State,
        ) {
        }
        fn on_after_step(&mut self, experiences: &Experience<Self>) {}
    }

}
mod sdl2 {
    pub fn sine_carve() {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        use sdl2::rect::Point;
        use sdl2::rect::Rect;
        use std::time::Duration;
        use try_from::*;
        let width = 640;
        let height = 480;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("SDL2", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            // .present_vsync()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let texture_creator = canvas.texture_creator();

        let mut timer = sdl_context.timer().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let width_f = Into::<f64>::into(width);
        let height_f = Into::<f64>::into(height);
        let mut phase_shift: f64 = 0.0_f64;
        let f_y = |x: f64, phase_shift| -> f64 { f64::sin((x + phase_shift) / 100.0_f64) };
        let scale = |y: f64| -> i32 { ((y + 1.0_f64) * (height_f / 2.0_f64)) as i32 };

        let mut running = true;
        while running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        running = false;
                    }
                    _ => {}
                }
            }
            let ticks: i32 = timer.ticks().try_into().unwrap();

            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
            canvas.fill_rect(Rect::new(10, 10, 10, 10)).unwrap();

            for i in 0..width - 1 {
                let start_x: f64 = i.into();
                let start_y: f64 = f_y(start_x, phase_shift);
                let end_x: f64 = (i + 1).into();
                let end_y: f64 = f_y(end_x, phase_shift);
                let sx = start_x as i32;
                let sy = scale(start_y);
                let ex = start_x as i32;
                let ey = scale(end_y);
                let start = Point::new(sx, sy);
                let end = Point::new(ex, ey);

                canvas.draw_line(start, end).unwrap();
            }
            phase_shift += 10.0_f64;
            canvas.present();
            std::thread::sleep(Duration::from_millis(15));
        }
    }
}

fn main() {
    sdl2::sine_carve();
}
