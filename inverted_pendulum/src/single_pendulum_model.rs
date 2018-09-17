extern crate ndarray;
extern crate ndarray_rand;

use ndarray::prelude::*;

use std::f64::consts::PI;

const GRAVITATIONAL_ACCELERATION: f64 = 9.80665_f64; // m/s^2
const LENGTH: f64 = 1_f64; // m
const FRICTION_LOSS: f64 = 0.1_f64;

/// 単振り子の運動方程式
/// d2θ/dt2 = -(g/l)sinθ
fn acceleration_of_theta1(current_theta: f64) -> f64 {
    let acceleration_of_theta = GRAVITATIONAL_ACCELERATION / LENGTH * current_theta.sin();
    acceleration_of_theta
}

/// http://godfoot.world.coocan.jp/furiko.htm
/// ルンゲクッタ(ode45)は１階連立微分方程式の解法なので、１階連立微分方程式に変換する。
/// d2θ/dt2 = (-g/l)θ
/// を
/// dθ/dt = ω (角速度)
/// dω/dt = -(g/l)sinθ (各加速度)
/// の１階連立微分方程式にする。
/// そして関数化
/// F( t , ω, θ) =　(-g/l)sinθ
/// G( t , ω, θ) = ω

fn f(t: f64, _omega: f64, theta: f64) -> f64 {
    ((-GRAVITATIONAL_ACCELERATION) / LENGTH) * theta.sin()
}
fn g(_t: f64, omega: f64, _theta: f64) -> f64 {
    omega
}



fn main() {
    let delta_t = 0.01_f64; // sec
    let t0 = 0_f64;
    let omega0 = 0_f64;
    let theta0 = PI * 0.99;
    let mut time = t0;
    let mut omega = omega0;
    let mut theta = theta0;
    let mut count = 0;
    for step in 0..100000 {
        let degree = theta * 180_f64 / PI;
        // 収束判定
        if  degree*degree < 0.01_f64 {
            count += 1;
            if count > 4 {
                break;
            }
        } else {
            count = 0
        }
        println!("{}\t{}\t{}\t{}", step, time, theta, degree);
        // https://ja.wikipedia.org/wiki/%E3%83%AB%E3%83%B3%E3%82%B2%EF%BC%9D%E3%82%AF%E3%83%83%E3%82%BF%E6%B3%95
        // k1(n) = F(
        //     t(n),
        //     y(n)
        // )
        let k1 = f(time, omega, theta);
        let m1 = g(time, omega, theta);
        // k2(n) = F(
        //     t(n) +         dt/2,
        //     y(n) + k1(n) * dt/2
        // )
        let k2 = f(
            time + delta_t / 2_f64,
            omega + k1 * delta_t / 2_f64,
            theta + m1 * delta_t / 2_f64,
        );
        let m2 = g(
            time + delta_t / 2_f64,
            omega + k1 * delta_t  / 2_f64,
            theta + m1 * delta_t  / 2_f64,
        );
        // k3(n) = f(
        //      t(n) +         dt/2,
        //      y(n) + k2(n) * dt/2
        // )
        let k3 = f(
            time + delta_t / 2_f64,
            omega + k2 * delta_t / 2_f64,
            theta + m2 * delta_t  / 2_f64,
        );
        let m3 = g(
            time + delta_t / 2_f64,
            omega + k2 * delta_t  / 2_f64,
            theta + m2 * delta_t  / 2_f64,
        );
        // k4(n) = f(
        //      t(n) +         dt,
        //      y(n) + k3(n) * dt
        // )
        let k4 = f(
            time + delta_t,
            omega + k3 * delta_t,
            theta + m3 * delta_t
        );
        let m4 = g(
            time + delta_t,
            omega + k3 * delta_t,
            theta + m3 * delta_t
        );
        // k = ( k1(n) + 2*k2(n) + 2*k3(n) + k4(n) ) * dt/6
        let k = (k1 + 2_f64 * k2 + 2_f64 * k3 + k4) * delta_t / 6_f64;
        let m = (m1 + 2_f64 * m2 + 2_f64 * m3 + m4) * delta_t / 6_f64;
        // y(n+1) = y(n) + k
        omega = omega + k;
        // http://tyk-systems.com/PandaFrequency/PandaFrequency.html
        omega = omega - omega*0.0001; // 角速度に比例する抵抗
        theta = theta + m;
        time = time + delta_t;
    }
}
