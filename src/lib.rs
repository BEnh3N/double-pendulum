use nannou::prelude::*;

const NUM_ARGS: usize = 7;
const G: f64 = 9.81;

pub struct DoublePendulum {
    pub t1: f64,
    pub v1: f64,
    pub t2: f64,
    pub v2: f64,
    pub a1: f64,
    pub a2: f64,

    pub t: f64,

    pub m1: f64,
    pub m2: f64,
    pub l1: f64,
    pub l2: f64,

    pub col: Hsla,
}

impl DoublePendulum {
    pub fn get_vars(&self) -> [f64; NUM_ARGS] {
        [self.t1, self.v1, self.t2, self.v2, self.a1, self.a2, self.t]
    }

    pub fn set_vars(&mut self, vars: [f64; NUM_ARGS]) {
        self.t1 = vars[0];
        self.v1 = vars[1];
        self.t2 = vars[2];
        self.v2 = vars[3];
        self.a1 = vars[4];
        self.a2 = vars[5];
        self.t = vars[6];
    }
}

impl Default for DoublePendulum {
    fn default() -> Self {
        Self {
            t1: PI_F64 / 2.,
            v1: 0.,
            t2: PI_F64 / 2.,
            v2: 0.,
            a1: 0.,
            a2: 0.,

            t: 0.,

            m1: 2.,
            m2: 2.,
            l1: 1.,
            l2: 1.,

            col: hsla(1.0, 1.0, 1.0, 1.0),
        }
    }
}

pub struct Model {
    pub pendulums: Vec<DoublePendulum>,
    pub prev_time: f64,
    pub step_forward: bool,
    pub step: bool,
    // _window: window::Id,
}

pub fn limit_angle(angle: f64) -> f64 {
    if angle > PI_F64 {
        let n = ((angle - -PI_F64) / (2. * PI_F64)).floor();
        angle - 2. * PI_F64 * n
    } else if angle < -PI_F64 {
        let n = (-(angle - PI_F64) / (2. * PI_F64)).floor();
        angle + 2. * PI_F64 * n
    } else {
        angle
    }
}

pub fn runge_kutta_step(pendulum: &mut DoublePendulum, step_size: f64) {
    let mut vars = pendulum.get_vars();

    let mut inp = vars.clone();

    let mut k1 = [0.; NUM_ARGS];
    evaluate(&pendulum, &inp, &mut k1, 0.);

    for i in 0..NUM_ARGS {
        inp[i] = vars[i] + k1[i] * step_size / 2.;
    }

    // ----
    let mut k2 = [0.; NUM_ARGS];
    evaluate(&pendulum, &inp, &mut k2, step_size / 2.);

    for i in 0..NUM_ARGS {
        inp[i] = vars[i] + k2[i] * step_size / 2.;
    }

    // ----
    let mut k3 = [0.; NUM_ARGS];
    evaluate(&pendulum, &inp, &mut k3, step_size / 2.);

    for i in 0..NUM_ARGS {
        inp[i] = vars[i] + k3[i] * step_size;
    }

    // ----
    let mut k4 = [0.; NUM_ARGS];
    evaluate(&pendulum, &inp, &mut k4, step_size);

    for i in 0..NUM_ARGS {
        vars[i] += (k1[i] + 2. * k2[i] + 2. * k3[i] + k4[i]) * step_size / 6.;
    }

    pendulum.set_vars(vars);
}

pub fn evaluate(
    pendulum: &DoublePendulum,
    vars: &[f64; NUM_ARGS],
    change: &mut [f64; NUM_ARGS],
    _time_step: f64,
) {
    change.fill(0.);
    change[NUM_ARGS - 1] = 1.;

    let th1 = vars[0];
    let dth1 = vars[1];
    let th2 = vars[2];
    let dth2 = vars[3];
    let m1 = pendulum.m1;
    let m2 = pendulum.m2;
    let l1 = pendulum.l1;
    let l2 = pendulum.l2;

    change[0] = dth1;

    let mut num = -G * (2. * m1 + m2) * th1.sin();
    num = num - G * m2 * (th1 - 2. * th2).sin();
    num = num - 2. * m2 * dth2 * dth2 * l2 * (th1 - th2).sin();
    num = num - m2 * dth1 * dth1 * l1 * (2. * (th1 - th2)).sin();
    num = num / (l1 * (2. * m1 + m2 - m2 * (2. * (th1 - th2)).cos()));
    change[1] = num;

    change[2] = dth2;

    num = (m1 + m2) * dth1 * dth1 * l1;
    num = num + G * (m1 + m2) * th1.cos();
    num = num + m2 * dth2 * dth2 * l2 * (th1 - th2).cos();
    num = num * 2. * (th1 - th2).sin();
    num = num / (l2 * (2. * m1 + m2 - m2 * (2. * (th1 - th2)).cos()));
    change[3] = num;
}
