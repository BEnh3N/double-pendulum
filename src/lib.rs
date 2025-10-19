use bevy::prelude::*;
use bevy_egui::egui::epaint::Hsva;

pub mod dp;
pub mod ui;
use std::f64::consts::PI;

use dp::*;

pub const G: f64 = 9.81;
pub const LINE_MUL: f32 = 150.0;
pub const RAD_TO_DEG: f64 = 180.0 / PI;
pub const PI2: f64 = PI * 2.0;
pub const PI05: f64 = PI / 2.0;

#[derive(Resource)]
pub struct TimeRate(pub f64);

impl Default for TimeRate {
    fn default() -> Self {
        TimeRate(1.0)
    }
}

#[derive(Resource)]
pub struct Points(Vec<Vec<[f64; 2]>>);

impl Points {
    pub fn empty(&mut self) {
        self.0 = vec![vec![]]
    }

    pub fn add_line(&mut self) {
        self.0.push(vec![]);
    }

    pub fn push(&mut self, pt: [f64; 2]) {
        let i = self.0.len() - 1;
        self.0[i].push(pt);
    }
}

impl Default for Points {
    fn default() -> Self {
        Points(vec![vec![]])
    }
}

#[derive(Resource)]
pub struct StepForward {
    pub enabled: bool,
    pub time_step: f64,
    pub step: bool,
}

impl Default for StepForward {
    fn default() -> Self {
        StepForward {
            enabled: false,
            time_step: 0.01,
            step: false,
        }
    }
}

pub fn initialize_pendulums(
    commands: &mut Commands,
    num_pendulums: u32,
    start_angle: f64,
    offset_angle: f64,
    hue: f32,
) {
    let mut angle = start_angle;
    for i in 0..num_pendulums {
        let s = i as f32 / num_pendulums as f32;
        commands.spawn(DoublePendulum::new(
            angle,
            angle,
            Hsva::new(hue, 1.0 - s, 1.0, 1.0).into(),
        ));
        angle += offset_angle;
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
    let m1 = pendulum.p1.mass;
    let m2 = pendulum.p2.mass;
    let l1 = pendulum.p1.length;
    let l2 = pendulum.p2.length;

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
