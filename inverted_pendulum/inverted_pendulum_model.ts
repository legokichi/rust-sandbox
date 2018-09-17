// https://jsfiddle.net/7fnmgu8L/56/
const GRAVITATIONAL_ACCELERATION = -9.80665; // m/s^2
const LENGTH_OF_PENDULUM = 1;
const MASS_OF_PENDULUM = 1;
const MASS_OF_CAR = 1;
const PI = Math.PI;

// http://www.rel.hiroshima-u.ac.jp/inverted/index.html
class InvertedPendulum {
  constructor(
    /// 時刻
    public time: number,
    /// 位置
    public x: number,
    /// 速度
    public v: number,
    /// 角度
    public theta: number,
    /// 角速度
    public omega: number,
    /// 外力
    public outer_force: number,
  ) {
  }
  /// (d/dt)x = v
  public velocity(): number {
    return this.v;
  }
  /// (d/dt)v =  ...
  public acceleration(): number {
    return (
      (
        this.outer_force + LENGTH_OF_PENDULUM * Math.pow(this.omega, 2) * Math.sin(this.theta)
      ) - (
        MASS_OF_PENDULUM * GRAVITATIONAL_ACCELERATION * Math.sin(this.theta) * Math.cos(this.theta)
      )
    ) / (
        MASS_OF_CAR + MASS_OF_PENDULUM * Math.sin(Math.sin(this.theta))
      );
  }
  /// (d/dt)theta = omega
  public angular_velocity(): number {
    return this.omega;
  }
  /// (d/dt)omega = ...
  public angular_acceleration(): number {
    return (
      (
        (MASS_OF_CAR + MASS_OF_PENDULUM) * GRAVITATIONAL_ACCELERATION * Math.sin(this.theta)
      ) - (
        (this.outer_force + (LENGTH_OF_PENDULUM * Math.pow(this.omega, 2) * Math.sin(this.theta))) * Math.cos(this.theta)
      )
    ) / (
        LENGTH_OF_PENDULUM * (
          MASS_OF_CAR +
          MASS_OF_PENDULUM * Math.sin(Math.sin(this.theta))
        )
      );
  }
  /// 加速度
  public a(): number {
    return this.acceleration();
  }
  /// 角加速度
  public alpha(): number {
    return this.angular_acceleration();
  }
  public degree(): number {
    return this.theta * 180 / PI;
  }
  public runge_kutta(delta_t: number): InvertedPendulum {
    // k1(n) = F(
    //     t(n),
    //     y(n)
    // )
    const param1 = this;
    const k1 = param1.velocity();
    const l1 = param1.acceleration();
    const m1 = param1.angular_velocity();
    const n1 = param1.angular_acceleration();
    // k2(n) = F(
    //     t(n) +         dt/2,
    //     y(n) + k1(n) * dt/2
    // )
    const param2 = new InvertedPendulum(
      this.time + delta_t / 2,
      this.x + k1 * delta_t / 2,
      this.v + l1 * delta_t / 2,
      this.theta + m1 * delta_t / 2,
      this.omega + n1 * delta_t / 2,
      this.outer_force,
    );
    const k2 = param2.velocity();
    const l2 = param2.acceleration();
    const m2 = param2.angular_velocity();
    const n2 = param2.angular_acceleration();
    // k3(n) = f(
    //      t(n) +         dt/2,
    //      y(n) + k2(n) * dt/2
    // )
    const param3 = new InvertedPendulum(
      this.time + delta_t / 2,
      this.x + k2 * delta_t / 2,
      this.v + l2 * delta_t / 2,
      this.theta + m2 * delta_t / 2,
      this.omega + n2 * delta_t / 2,
      this.outer_force,
    );
    const k3 = param3.velocity();
    const l3 = param3.acceleration();
    const m3 = param3.angular_velocity();
    const n3 = param3.angular_acceleration();
    // k4(n) = f(
    //      t(n) +         dt,
    //      y(n) + k3(n) * dt
    // )
    const param4 = new InvertedPendulum(
      this.time + delta_t,
      this.x + k3 * delta_t,
      this.v + l3 * delta_t,
      this.theta + m3 * delta_t,
      this.omega + n3 * delta_t,
      this.outer_force,
    );
    const k4 = param4.velocity();
    const l4 = param4.acceleration();
    const m4 = param4.angular_velocity();
    const n4 = param4.angular_acceleration();
    // k = ( k1(n) + 2*k2(n) + 2*k3(n) + k4(n) ) * dt/6
    const k = (k1 + 2 * k2 + 2 * k3 + k4) * delta_t / 6;
    const l = (l1 + 2 * l2 + 2 * l3 + l4) * delta_t / 6;
    const m = (m1 + 2 * m2 + 2 * m3 + m4) * delta_t / 6;
    const n = (n1 + 2 * n2 + 2 * n3 + n4) * delta_t / 6;
    // y(n+1) = y(n) + k
    return new InvertedPendulum(
      this.time + delta_t,
      this.x + k,
      this.v + l,
      this.theta + m,
      this.omega + n,
      this.outer_force,
    );
  }
}

function main() {
  console.clear();
  const cnv = document.createElement("canvas");
  cnv.style.border = "1px solid black";
  cnv.width = 800;
  cnv.height = 400;
  document.body.appendChild(cnv);
  const ctx = cnv.getContext("2d");

  const delta_t = 0.01; // sec
  let state = new InvertedPendulum(
    0,
    0,
    0,
    PI * 0.99,
    0,
    0,
  );
  function recur() {
    // clear
    //cnv.width = cnv.width;

    // horizon
    ctx.strokeStyle = "black";
    ctx.beginPath();
    ctx.moveTo(0, cnv.height / 2);
    ctx.lineTo(cnv.width, cnv.height / 2);
    ctx.stroke();

    const x = state.x;
    const theta = state.theta;
    const length = cnv.height / 8;
    //console.log(state.time, x, theta, state.degree());

    ctx.strokeStyle = "hsl(" + state.time % 4 / 4 * 360 + ",80%,50%)";

    // car
    const rect_width = 30;
    const rect_height = 15;
    ctx.rect(cnv.width / 2 - rect_width / 2 + x, cnv.height / 2 - rect_height, rect_width, rect_height);
    ctx.stroke();

    // pendulum

    ctx.beginPath();
    ctx.moveTo(cnv.width / 2 + x, cnv.height / 2 - rect_height);
    ctx.lineTo(cnv.width / 2 + x - Math.cos(theta - PI / 2) * length, cnv.height / 2 - Math.sin(theta - PI / 2) * length - rect_height);
    ctx.closePath();
    ctx.stroke();

    // update
    state = state.runge_kutta(delta_t);
    requestAnimationFrame(recur);
  }
  requestAnimationFrame(recur);

}


main()