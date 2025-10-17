use nannou::{color, prelude::*};

pub mod dp;
use dp::*;
use nannou_egui::egui::epaint::Hsva;

pub mod ui;

pub const G: f64 = 9.81;
pub const RAD_TO_DEG: f64 = 57.2958;
pub const LINE_MUL: f32 = 175.;
const PI2: f32 = 2.0 * PI;

pub struct Model {
    pub egui: nannou_egui::Egui,
    pub pendulums: Vec<DoublePendulum>,
    pub limit_angles: bool,
    pub time_rate: f64,
    pub time_step: f64,
    pub g: f64,
    pub step_forward: bool,
    pub step: bool,

    pub points: Vec<Vec<[f64; 2]>>,
    pub initial_state: Vec<DoublePendulum>,
}

impl Model {
    pub fn reset(&mut self) {
        self.pendulums = self.initial_state.clone();
        self.points = vec![vec![]]
    }
}

pub fn limit_angle(angle: f64) -> (f64, bool) {
    if angle > PI_F64 {
        let n = ((angle - -PI_F64) / (2. * PI_F64)).floor();
        (angle - 2. * PI_F64 * n, true)
    } else if angle < -PI_F64 {
        let n = (-(angle - PI_F64) / (2. * PI_F64)).floor();
        (angle + 2. * PI_F64 * n, true)
    } else {
        (angle, false)
    }
}

pub fn runge_kutta_step(pendulum: &mut DoublePendulum, step_size: f64) {
    let mut vars = pendulum.get_vars();

    let mut inp = vars;

    let mut k1 = [0.; NUM_ARGS];
    evaluate(pendulum, &inp, &mut k1, 0.);

    for i in 0..NUM_ARGS {
        inp[i] = vars[i] + k1[i] * step_size / 2.;
    }

    // ----
    let mut k2 = [0.; NUM_ARGS];
    evaluate(pendulum, &inp, &mut k2, step_size / 2.);

    for i in 0..NUM_ARGS {
        inp[i] = vars[i] + k2[i] * step_size / 2.;
    }

    // ----
    let mut k3 = [0.; NUM_ARGS];
    evaluate(pendulum, &inp, &mut k3, step_size / 2.);

    for i in 0..NUM_ARGS {
        inp[i] = vars[i] + k3[i] * step_size;
    }

    // ----
    let mut k4 = [0.; NUM_ARGS];
    evaluate(pendulum, &inp, &mut k4, step_size);

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
    num -= G * m2 * (th1 - 2. * th2).sin();
    num -= 2. * m2 * dth2 * dth2 * l2 * (th1 - th2).sin();
    num -= m2 * dth1 * dth1 * l1 * (2. * (th1 - th2)).sin();
    num /= l1 * (2. * m1 + m2 - m2 * (2. * (th1 - th2)).cos());
    change[1] = num;

    change[2] = dth2;

    num = (m1 + m2) * dth1 * dth1 * l1;
    num += G * (m1 + m2) * th1.cos();
    num += m2 * dth2 * dth2 * l2 * (th1 - th2).cos();
    num *= 2. * (th1 - th2).sin();
    num /= l2 * (2. * m1 + m2 - m2 * (2. * (th1 - th2)).cos());
    change[3] = num;
}

pub fn initialize_pendulums(
    num_pendulums: u32,
    start_angle: f64,
    offset_angle: f64,
    hue: f32,
) -> Vec<DoublePendulum> {
    let mut pendulums = vec![];

    let mut angle = start_angle;
    for i in 0..num_pendulums {
        let s = i as f32 / num_pendulums as f32;
        pendulums.push(
            // 0,
            DoublePendulum {
                t1: angle,
                t2: angle,
                col: Hsva::new(hue, s * 3., 1. - s, 1.0),
                ..Default::default()
            },
        );
        angle += offset_angle;
    }

    pendulums.reverse();
    pendulums
}

pub fn hsva_rad(h: f32, s: f32, v: f32, a: f32) -> color::Hsva {
    hsva(h / (PI2), s, v, a)
}

pub fn calc_standard_dev(values: &Vec<f64>) -> f64 {
    let mut sum = 0.;
    for value in values {
        sum += value;
    }
    let mean = sum / values.len() as f64;

    sum = 0.;
    for value in values {
        sum += (value - mean).powi(2);
    }

    (sum / values.len() as f64).sqrt()
}
