use bevy::prelude::*;
use bevy_egui::egui::epaint::Hsva;

pub mod dp;
pub mod ui;
use std::f64::consts::PI;

use dp::*;

pub const LINE_MUL: f32 = 150.0;
pub const RAD_TO_DEG: f64 = 180.0 / PI;
pub const PI2: f64 = PI * 2.0;
pub const PI05: f64 = PI / 2.0;

#[derive(Resource)]
pub struct Gravity(pub f64);

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
            Hsva::new(hue, 1.0 - s, s.sqrt(), 1.0),
        ));
        angle += offset_angle;
    }
}

pub fn runge_kutta_step(pendulum: &mut DoublePendulum, h: f64, g: f64) {
    let m1 = pendulum.p1.mass;
    let m2 = pendulum.p2.mass;
    let l1 = pendulum.p1.length;
    let l2 = pendulum.p2.length;

    let state = (
        pendulum.p1.angle,
        pendulum.p1.velocity,
        pendulum.p2.angle,
        pendulum.p2.velocity,
    );

    let next_state = rk4(state, h, |(a1, v1, a2, v2)| {
        let (acc1, acc2) = acceleration(a1, v1, a2, v2, g, m1, m2, l1, l2);
        (v1, acc1, v2, acc2)
    });

    pendulum.p1.angle = next_state.0;
    pendulum.p1.velocity = next_state.1;
    pendulum.p2.angle = next_state.2;
    pendulum.p2.velocity = next_state.3;
}

fn acceleration(
    a1: f64,
    v1: f64,
    a2: f64,
    v2: f64,
    g: f64,
    m1: f64,
    m2: f64,
    l1: f64,
    l2: f64,
) -> (f64, f64) {
    let delta = a1 - a2;
    let sin_delta = delta.sin();
    let cos_delta = delta.cos();
    let sin_2delta = (2.0 * delta).sin();
    let cos_2delta = (2.0 * delta).cos();
    let denom = 2.0 * m1 + m2 - m2 * cos_2delta;

    let acc1 = (-g * (2. * m1 + m2) * a1.sin()
        - g * m2 * (a1 - 2. * a2).sin()
        - 2. * m2 * v2 * v2 * l2 * sin_delta
        - m2 * v1 * v1 * l1 * sin_2delta)
        / (l1 * denom);

    let acc2 = (2.0
        * sin_delta
        * (v1 * v1 * l1 * (m1 + m2) + g * (m1 + m2) * a1.cos() + v2 * v2 * l2 * m2 * cos_delta))
        / (l2 * denom);

    (acc1, acc2)
}

fn rk4<F>(state: (f64, f64, f64, f64), h: f64, derivs: F) -> (f64, f64, f64, f64)
where
    F: Fn((f64, f64, f64, f64)) -> (f64, f64, f64, f64),
{
    let (s1, s2, s3, s4) = state;

    let (k1_1, k1_2, k1_3, k1_4) = derivs((s1, s2, s3, s4));
    let (k2_1, k2_2, k2_3, k2_4) = derivs((
        s1 + 0.5 * h * k1_1,
        s2 + 0.5 * h * k1_2,
        s3 + 0.5 * h * k1_3,
        s4 + 0.5 * h * k1_4,
    ));
    let (k3_1, k3_2, k3_3, k3_4) = derivs((
        s1 + 0.5 * h * k2_1,
        s2 + 0.5 * h * k2_2,
        s3 + 0.5 * h * k2_3,
        s4 + 0.5 * h * k2_4,
    ));
    let (k4_1, k4_2, k4_3, k4_4) =
        derivs((s1 + h * k3_1, s2 + h * k3_2, s3 + h * k3_3, s4 + h * k3_4));

    (
        s1 + h / 6.0 * (k1_1 + 2.0 * k2_1 + 2.0 * k3_1 + k4_1),
        s2 + h / 6.0 * (k1_2 + 2.0 * k2_2 + 2.0 * k3_2 + k4_2),
        s3 + h / 6.0 * (k1_3 + 2.0 * k2_3 + 2.0 * k3_3 + k4_3),
        s4 + h / 6.0 * (k1_4 + 2.0 * k2_4 + 2.0 * k3_4 + k4_4),
    )
}

pub fn key_press(
    keys: Res<ButtonInput<KeyCode>>,
    mut pendulums: Query<&mut DoublePendulum>,
    mut points: ResMut<Points>,
    mut step_forward: ResMut<StepForward>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        pendulums.iter_mut().for_each(|mut p| p.reset());
        points.empty();
    }
    if keys.just_pressed(KeyCode::ShiftLeft) {
        step_forward.enabled = !step_forward.enabled;
    }
    if step_forward.enabled && keys.just_pressed(KeyCode::Space) {
        step_forward.step = true;
    }
}
