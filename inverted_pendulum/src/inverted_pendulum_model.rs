const GRAVITATIONAL_ACCELERATION: f64 = 9.80665_f64; // m/s^2
const LENGTH_OF_PENDULUM: f64 = 1_f64;
const MASS_OF_PENDULUM: f64 = 1_f64;
const MASS_OF_CAR: f64 = 1_f64;
use std::f64::consts::PI;

#[derive(Debug)]
struct InvertedPendulum {
    /// 時刻
    time: f64,
    /// 位置
    x: f64,
    /// 速度
    v: f64,
    /// 角度
    theta: f64,
    /// 角速度
    omega: f64,
    /// 外力
    outer_force: f64,
}

impl InvertedPendulum {
    /// (d/dt)x = v
    pub fn velocity(&self) -> f64 {
        self.v
    }
    /// (d/dt)v =  ...
    pub fn acceleration(&self) -> f64 {
        (
            (
                self.outer_force + LENGTH_OF_PENDULUM * self.omega.powi(2) * self.theta.sin()
            ) - (
                MASS_OF_PENDULUM * GRAVITATIONAL_ACCELERATION * self.theta.sin() * self.theta.cos()
            )
        ) / (
            MASS_OF_CAR + MASS_OF_PENDULUM * self.theta.sin().sin()
        )
    }
    /// (d/dt)theta = omega
    pub fn angular_velocity(&self) -> f64 {
        self.omega
    }
    /// (d/dt)omega = ...
    pub fn angular_acceleration(&self) -> f64 {
        (
            (
                (MASS_OF_CAR + MASS_OF_PENDULUM) * GRAVITATIONAL_ACCELERATION * self.theta.sin()
            ) - (
                (self.outer_force + (LENGTH_OF_PENDULUM * self.omega.powi(2) * self.theta.sin())) * self.theta.cos() 
            )
        ) / (
            LENGTH_OF_PENDULUM * (
                MASS_OF_CAR + 
                MASS_OF_PENDULUM * self.theta.sin().sin()
            )
        )
    }
    /// 加速度
    pub fn a(&self) -> f64 {
        self.acceleration()
    }
    /// 角加速度
    pub fn alpha(&self) -> f64 {
        self.angular_acceleration()
    }
    pub fn degree(&self) -> f64 {
        let degree = self.theta * 180_f64 / PI;
        degree
    }
    pub fn runge_kutta(&self, delta_t: f64) -> Self {
        // k1(n) = F(
        //     t(n),
        //     y(n)
        // )
        let param1 = self;
        let k1 = param1.velocity();
        let l1 = param1.acceleration();
        let m1 = param1.angular_velocity();
        let n1 = param1.angular_acceleration();
        // k2(n) = F(
        //     t(n) +         dt/2,
        //     y(n) + k1(n) * dt/2
        // )
        let param2 = InvertedPendulum {
            time: self.time + delta_t / 2_f64,
            x: self.x + k1 * delta_t / 2_f64,
            v: self.v + l1 * delta_t / 2_f64,
            theta: self.theta + m1 * delta_t / 2_f64,
            omega: self.omega + n1 * delta_t / 2_f64,
            outer_force: self.outer_force,
        };
        let k2 = param2.velocity();
        let l2 = param2.acceleration();
        let m2 = param2.angular_velocity();
        let n2 = param2.angular_acceleration();
        // k3(n) = f(
        //      t(n) +         dt/2,
        //      y(n) + k2(n) * dt/2
        // )
        let param3 = InvertedPendulum {
            time: self.time + delta_t / 2_f64,
            x: self.x + k2 * delta_t / 2_f64,
            v: self.v + l2 * delta_t / 2_f64,
            theta: self.theta + m2 * delta_t  / 2_f64,
            omega: self.omega + n2 * delta_t / 2_f64,
            outer_force: self.outer_force,
        };
        let k3 = param3.velocity();
        let l3 = param3.acceleration();
        let m3 = param3.angular_velocity();
        let n3 = param3.angular_acceleration();
        // k4(n) = f(
        //      t(n) +         dt,
        //      y(n) + k3(n) * dt
        // )
        let param4 = InvertedPendulum {
            time: self.time + delta_t,
            x: self.x + k3 * delta_t,
            v: self.v + l3 * delta_t,
            theta: self.theta + m3 * delta_t,
            omega: self.omega + n3 * delta_t,
            outer_force: self.outer_force,
        };
        let k4 = param4.velocity();
        let l4 = param4.acceleration();
        let m4 = param4.angular_velocity();
        let n4 = param4.angular_acceleration();
        // k = ( k1(n) + 2*k2(n) + 2*k3(n) + k4(n) ) * dt/6
        let k = (k1 + 2_f64 * k2 + 2_f64 * k3 + k4) * delta_t / 6_f64;
        let l = (l1 + 2_f64 * l2 + 2_f64 * l3 + l4) * delta_t / 6_f64;
        let m = (m1 + 2_f64 * m2 + 2_f64 * m3 + m4) * delta_t / 6_f64;
        let n = (n1 + 2_f64 * n2 + 2_f64 * n3 + n4) * delta_t / 6_f64;
        // y(n+1) = y(n) + k
        InvertedPendulum {
            time: self.time + delta_t,
            x: self.x + k,
            v: self.v + l,
            theta: self.theta + m,
            omega: self.omega + n,
            outer_force: self.outer_force,
        }
    }
}

fn main() {
    let delta_t = 0.01_f64; // sec
    let mut state = InvertedPendulum{
        time: 0_f64,
        x: 0_f64,
        v: 0_f64,
        theta: PI * -0.99_f64,
        omega: 0_f64,
        outer_force: 0_f64,
    };
    for step in 0..100000 {
        state = state.runge_kutta(delta_t);
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}", step, state.x, state.v, state.acceleration(), state.theta, state.omega, state.alpha());
    }
}

