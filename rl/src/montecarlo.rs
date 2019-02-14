
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
// 行動の修正を実績に基づいて行う => モンテカルロ法
// 行動の修正を予測により行う => TD(0) 法
// この２つの間として TD(λ)法 と Multi-step Learning がある
// Multi-step Learning は最近の主流
// 
// モンテカルロ法 => エピソードが終了してから経験(実績)を戦略にフィードバックする
// TD(0)法 => 1 回の試行毎に経験を戦略にフィードバックするする

trait Environment: Debug+Clone+PartialEq+Eq+Hash {
    type Action: Action;
    fn reset(&mut self) -> ();
    fn step(&mut self, action: <Self::Action as Action>::SomeAction) -> (f64, bool);
    fn all_actions(&self) -> Vec<<Self::Action as Action>::SomeAction>;
}
trait Action: Debug+Copy+Clone+PartialEq+Eq+PartialOrd+Ord+Hash {
    type SomeAction: Debug+Copy+Clone+PartialEq+Eq+PartialOrd+Ord+Hash;
    fn all_actions(&self) -> Vec<Self::SomeAction>;
}

struct Agent<E: Environment> {
    epsilon: f64,
    Q: HashMap<E, HashMap<<E::Action as Action>::SomeAction, f64>>,
}
impl<E: Environment> Agent<E> {
    fn new(epsilon: f64) -> Self {
        let Q = HashMap::new();
        Self { epsilon, Q }
    }
}
impl<E: Environment> Agent<E> {
    fn policy(&mut self, state: &E) -> <E::Action as Action>::SomeAction {
        if !(rand::random::<f64>() < self.epsilon) {
            // 報酬獲得行動
            if let Some(ref actions_exprement) = self.Q.get(&state) {
                if !actions_exprement.is_empty() {
                    // この状況で過去に実行した選択肢の中で一番報酬が多かった選択肢を実行
                    let mut argmax_prob = 0.0;
                    let mut argmax_action = None;
                    for (action, prob) in actions_exprement.iter() {
                        if *prob >= argmax_prob {
                            argmax_prob = *prob;
                            argmax_action = Some(*action);
                        }
                    }
                    let some_action = argmax_action.unwrap();
                    // println!("報酬獲得行動: {:?}", some_action);
                    return some_action;
                }
            }
        }
        // 探索行動
        let actions = state.all_actions();
        use rand::seq::SliceRandom;
        // ランダムアクション
        let some_action = *actions.choose(&mut rand::thread_rng()).unwrap();
        // println!("探索行動: {:?}", some_action);
        some_action
    }
}
trait MonteCarlo<E: Environment> {
    fn learn(self, env: E, episode_count: usize, gamma: f64) -> f64;
}
impl<E: Environment> MonteCarlo<E> for Agent<E> {
    fn learn(mut self, mut env: E, episode_count: usize, gamma: f64) -> f64 {
        let mut total_rewards = vec![];
        let mut N: HashMap<E, HashMap<<E::Action as Action>::SomeAction, f64>> = HashMap::new();
        for epi in 1..episode_count {
            // まずは 1 episode 分の経験を積む
            env.reset();
            let mut experience = vec![];
            loop {
                let some_action = self.policy(&env);
                let (reward, done) = env.step(some_action);
                experience.push((env.clone(), some_action, reward));
                if done { break; }
            }
            // この episode における結果をポリシーに反映する
            for (i, exp) in experience.iter().enumerate() {
                let (state, some_action, reward) = exp;
                let mut G = 0.0_f64;
                let mut t = 0.0_f64;
                for j in i..experience.len() {
                    G += gamma.powf(t) * reward;
                    t += 1.0;
                }
                if let Some(o) = N.get_mut(&state) {
                    if o.get(&some_action).is_none() {
                        o.insert(*some_action, 0.0);
                    }
                }else{
                    let mut o = HashMap::new();
                    o.insert(*some_action, 0.0);
                    N.insert(state.clone(), o);
                }
                if let Some(o) = self.Q.get_mut(&state) {
                    if o.get(&some_action).is_none() {
                        o.insert(*some_action, 0.0);
                    }
                }else{
                    let mut o = HashMap::new();
                    o.insert(*some_action, 0.0);
                    self.Q.insert(state.clone(), o);
                }
                let tmp_n = N.get_mut(&state).unwrap()
                    .get_mut(&some_action).unwrap();
                *tmp_n += 1.0;
                let alpha = 1.0 / *tmp_n;
                let tmp_q = self.Q.get_mut(&state).unwrap()
                    .get_mut(&some_action).unwrap();
                *tmp_q += alpha * (G - *tmp_q);
            }
            let mut rewards = experience.into_iter().map(|(_, _, rewad)| rewad).collect::<Vec<_>>();
            use statrs::statistics::Variance;
            use statrs::statistics::Mean;
            let mean = rewards.mean();
            let std = rewards.std_dev();
            if epi % 1000 == 0{
                println!("episode: {}, mean: {}, std: {}", epi, mean, std);
            }
            total_rewards.append(&mut rewards);
        }
        println!("Q: {:?}", self.Q);
        total_rewards.iter().sum()
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
struct Coins(usize);
impl Action for Coins {
    type SomeAction = usize;
    fn all_actions(&self) -> Vec<Self::SomeAction> { (0..self.0).collect() }
}
#[derive(Debug,Clone)]
struct CoinToss {
    coins: Coins,
    head_probs: HashMap<<Coins as Action>::SomeAction, f64>,
    max_episode_steps: usize,
    toss_count: usize,
}
impl CoinToss {
    fn new(max_episode_steps: usize) -> Self {
        let mut head_probs = HashMap::new();
        let probs = (0..1000).map(|i| i as f64/1000.0).collect::<Vec<f64>>();
        for (i, prob) in probs.into_iter().enumerate() {
            head_probs.insert(i, prob);
        }
        let coins = Coins(head_probs.len());
        Self {
            coins,
            head_probs,
            max_episode_steps,
            toss_count: 0,
        }
    }
}
impl Hash for CoinToss {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // !!! コイン投げゲームは試行毎に state は変化しない !!!
        // self.max_episode_steps.hash(state);
        // self.toss_count.hash(state);
    }
}
impl PartialEq for CoinToss {
    fn eq(&self, other: &Self) -> bool {
        // !!! コイン投げゲームは試行毎に state は変化しない !!!
        // self.max_episode_steps == other.max_episode_steps && self.toss_count == other.toss_count
        true
    }
}
impl Eq for CoinToss {}
impl Environment for CoinToss {
    type Action = Coins;
    fn reset(&mut self) -> () {
        self.toss_count = 0;
    }
    fn step(&mut self, action: <Self::Action as Action>::SomeAction) -> (f64, bool) {
        let done = self.toss_count == self.max_episode_steps - 1;
        let head_prob = *self.head_probs.get(&action).unwrap();
        let rand: f64 = rand::random();
        let reward = if rand < head_prob { 1.0 } else { 0.0 };
        self.toss_count += 1;
        (reward, done)
    }
    fn all_actions(&self) -> Vec<<Self::Action as Action>::SomeAction> {
        self.coins.all_actions()
    }
}


fn main(){
    // epsilon の確率で環境の探索調査行動を行い、それ以外では 報酬獲得行動を取る
    let max_episode_steps = 10;
    for epsilon in vec![0.0, 0.1, 1.0] {
        let mut agent = Agent::<CoinToss>::new(epsilon);
        // max_episode_steps 内で 状態空間(この場合コイン数) を探索し尽くせれば epsilon = 0 でも 1 エピソードだけで学習が終わる!
        let env = CoinToss::new(max_episode_steps);
        let total_reward = agent.learn(env, 10000, 0.9);
        println!("epsilon: {}, {}", epsilon, total_reward);
    }
}

/*
問題が簡単過ぎて探索なし (epsilon = 0) でも報酬が獲得できてしまう

episode: 1000, mean: 0.7, std: 0.48304589153964794
episode: 2000, mean: 0.7, std: 0.48304589153964794
episode: 3000, mean: 0.7999999999999999, std: 0.4216370213557839
episode: 4000, mean: 0.6000000000000001, std: 0.5163977794943222
episode: 5000, mean: 0.6, std: 0.5163977794943222
episode: 6000, mean: 0.5, std: 0.5270462766947299
episode: 7000, mean: 0.6, std: 0.5163977794943222
episode: 8000, mean: 0.5, std: 0.5270462766947299
episode: 9000, mean: 0.5, std: 0.5270462766947299
Q: {CoinToss { coins: Coins(3), head_probs: {1: 0.2, 0: 0.1, 2: 0.6}, max_episode_steps: 10, toss_count: 1 }: {2: 2.4853768159992833, 0: 1.355, 1: 1.0855359331666667}}
epsilon: 0, 60015
episode: 1000, mean: 0.6, std: 0.5163977794943222
episode: 2000, mean: 0.5, std: 0.5270462766947299
episode: 3000, mean: 0.39999999999999997, std: 0.5163977794943222
episode: 4000, mean: 0.6, std: 0.5163977794943222
episode: 5000, mean: 0.8999999999999999, std: 0.31622776601683794
episode: 6000, mean: 0.8999999999999999, std: 0.31622776601683794
episode: 7000, mean: 0.5, std: 0.5270462766947299
episode: 8000, mean: 0.30000000000000004, std: 0.4830458915396479
episode: 9000, mean: 0.7, std: 0.48304589153964794
Q: {CoinToss { coins: Coins(3), head_probs: {0: 0.1, 1: 0.2, 2: 0.6}, max_episode_steps: 10, toss_count: 1 }: {2: 2.479204347246506, 1: 0.8406234215398539, 0: 0.4361388017163774}}
epsilon: 0.1, 56858
episode: 1000, mean: 0.20000000000000004, std: 0.4216370213557839
episode: 2000, mean: 0.39999999999999997, std: 0.5163977794943222
episode: 3000, mean: 0.19999999999999998, std: 0.4216370213557839
episode: 4000, mean: 0.20000000000000004, std: 0.42163702135578396
episode: 5000, mean: 0.19999999999999998, std: 0.4216370213557839
episode: 6000, mean: 0.20000000000000004, std: 0.4216370213557839
episode: 7000, mean: 0.3, std: 0.48304589153964794
episode: 8000, mean: 0.19999999999999998, std: 0.4216370213557839
episode: 9000, mean: 0.5, std: 0.5270462766947299
Q: {CoinToss { coins: Coins(3), head_probs: {0: 0.1, 2: 0.6, 1: 0.2}, max_episode_steps: 10, toss_count: 1 }: {1: 0.8310507356521278, 2: 2.496263470373337, 0: 0.404157655036972}}
epsilon: 1, 30081
*/
