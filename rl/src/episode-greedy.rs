#![allow(unused_variables)]

use std::collections::HashMap;


#[derive(Debug,Clone,PartialEq)]
struct CoinToss {
    head_probs: HashMap<Action, f64>,
    max_episode_steps: usize,
    toss_count: usize,
}
#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
enum Action {
    CoinA, CoinB, CoinC,
}
impl CoinToss {
    fn new(max_episode_steps: usize) -> Self {
        use Action::*;
        let mut head_probs = HashMap::new();
        head_probs.insert(CoinA, 0.1);
        head_probs.insert(CoinB, 0.8);
        head_probs.insert(CoinC, 0.3);
        Self {
            head_probs,
            max_episode_steps,
            toss_count: 0,
        }
    }
    fn reset(&mut self) {
        self.toss_count = 0;
    }
    fn len(&self) -> usize { self.head_probs.len() }
    fn step(&mut self, action: Action) -> (f64, bool) {
        let done = self.toss_count == self.max_episode_steps - 1;
        let head_prob = *self.head_probs.get(&action).unwrap();
        let rand: f64 = rand::random();
        let reward = if rand < head_prob { 1.0 } else { 0.0 };
        self.toss_count += 1;
        (reward, done)
    }
}
// epsilon の確率で環境の探索調査行動を行い、
// それ以外では 報酬獲得行動を取るアルゴリズム
struct EpsilonGreedyAgent {
    epsilon: f64,
    V: HashMap<Action, f64>,
}
impl EpsilonGreedyAgent {
    fn new(epsilon: f64) -> Self {
        use Action::*;
        let mut V: HashMap<Action, f64> = HashMap::new();
        V.insert(CoinA, 0.0);
        V.insert(CoinB, 0.0);
        V.insert(CoinC, 0.0);
        Self { epsilon, V }
    }
    fn policy(&self) -> Action{
        use Action::*;
        if rand::random::<f64>() < self.epsilon {
            // 探索行動
            let coins = vec![CoinA, CoinB, CoinC];
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            let coin = *coins.choose(&mut rng).unwrap();
            coin
        } else {
            // 報酬獲得行動
            let (coin, prob) = self.V.iter().fold((CoinA, *self.V.get(&CoinA).unwrap()), |(act, preb), (action, prob)|{
                if preb < *prob {
                    (*action, *prob)
                } else {
                    (act, preb)
                }
            });
            coin
        }
    }
    fn play(&mut self, env: &mut CoinToss) -> Vec<f64> {
        use Action::*;
        let mut N: HashMap<Action, usize> = HashMap::new();
        N.insert(CoinA, 0);
        N.insert(CoinB, 0);
        N.insert(CoinC, 0);
        env.reset();
        let mut rewards = vec![];
        loop {
            // 1 エピソード終了するまで試し、得られた経験を反映していく
            let selected_coin = self.policy();
            let (reward, done) = env.step(selected_coin);
            rewards.push(reward);
            let n = *N.get(&selected_coin).unwrap();
            let coin_average = *self.V.get(&selected_coin).unwrap();
            let new_average = (coin_average * (n as f64) + reward) / ((n + 1) as f64);
            *N.get_mut(&selected_coin).unwrap() += 1;
            *self.V.get_mut(&selected_coin).unwrap() = new_average;
            if done { break; }
        }
        rewards
    }
}

fn main(){
    // 無限のエピソード試行回数がある場合 0 に近づくほど総和は大きくなるが
    // 有限の試行回数では 0.1 付近が最も効率がよい
    // = 行動のうち 90% を調査に向け、残りの 10% で報酬獲得を目指す戦略
    for epsilon in vec![0.0_f64, 0.05, 0.075, 0.1, 0.125, 0.15, 0.2, 0.4, 0.7, 0.9] {
        let mut agent = EpsilonGreedyAgent::new(epsilon);
        let mut means = vec![];
        for i in 0..100000 {
            let mut env = CoinToss::new(1000);
            let rewards = agent.play(&mut env);
            means.push(math::mean::arithmetic(&rewards));
        }
        println!("{:?}", means.into_iter().sum::<f64>());
    }
}