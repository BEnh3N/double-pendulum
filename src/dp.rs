use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_egui::egui::epaint::Hsva;

use crate::{PI05, PI2};

pub const NUM_ARGS: usize = 7;

#[derive(Component, Default)]
pub struct DoublePendulum {
    pub p1: Pendulum,
    pub p2: Pendulum,
    pub col: Hsva,
    pub t: f64,

    pub initial_state: (Pendulum, Pendulum),
}

impl DoublePendulum {
    pub fn new(a1: f64, a2: f64, col: Hsva) -> DoublePendulum {
        let p1 = Pendulum {
            angle: a1,
            ..Default::default()
        };
        let p2 = Pendulum {
            angle: a2,
            ..Default::default()
        };
        let initial_state = (p1.clone(), p2.clone());
        DoublePendulum {
            p1,
            p2,
            col,
            t: 0.0,
            initial_state,
        }
    }

    pub fn reset(&mut self) {
        self.p1 = self.initial_state.0.clone();
        self.p2 = self.initial_state.1.clone();
    }

    pub fn get_vars(&self) -> [f64; NUM_ARGS] {
        [
            self.p1.angle,
            self.p1.velocity,
            self.p2.angle,
            self.p2.velocity,
            self.p1.acceleration,
            self.p2.acceleration,
            self.t,
        ]
    }

    pub fn set_vars(&mut self, vars: [f64; NUM_ARGS]) {
        self.p1.angle = vars[0];
        self.p1.velocity = vars[1];
        self.p2.angle = vars[2];
        self.p2.velocity = vars[3];
        self.p1.acceleration = vars[4];
        self.p2.acceleration = vars[5];
        self.t = vars[6];
    }
}

#[derive(Clone)]
pub struct Pendulum {
    pub angle: f64,
    pub velocity: f64,
    pub acceleration: f64,

    pub length: f64,
    pub mass: f64,
}

impl Pendulum {
    pub fn clamp(&mut self) -> bool {
        let t = &mut self.angle;
        if *t > PI {
            *t = (*t + PI) % PI2 - PI;
            true
        } else if *t < -PI {
            *t = (*t + PI) % PI2 + PI;
            true
        } else {
            false
        }
    }
}

impl Default for Pendulum {
    fn default() -> Self {
        Pendulum {
            angle: PI05,
            velocity: 0.0,
            acceleration: 0.0,
            length: 1.0,
            mass: 2.0,
        }
    }
}
