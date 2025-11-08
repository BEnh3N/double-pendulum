use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_egui::egui::epaint::Hsva;

use crate::{acceleration, rk4, PI05, PI2};

#[derive(Component, Default)]
pub struct DoublePendulum {
    pub p1: Pendulum,
    pub p2: Pendulum,
    pub col: Hsva,
    pub t: f64,

    pub initial_state: (Pendulum, Pendulum),
}

impl DoublePendulum {
    pub fn new<C>(a1: f64, a2: f64, col: C) -> DoublePendulum
    where
        C: Into<Hsva>,
    {
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
            col: col.into(),
            t: 0.0,
            initial_state,
        }
    }

    pub fn step(&mut self, time_step: f64, g: f64) {
        let m1 = self.p1.mass;
        let m2 = self.p2.mass;
        let l1 = self.p1.length;
        let l2 = self.p2.length;

        let state = (
            self.p1.angle,
            self.p1.velocity,
            self.p2.angle,
            self.p2.velocity,
        );

        let next_state = rk4(state, time_step, |(a1, v1, a2, v2)| {
            let (acc1, acc2) = acceleration(a1, v1, a2, v2, g, m1, m2, l1, l2);
            (v1, acc1, v2, acc2)
        });

        self.p1.angle = next_state.0;
        self.p1.velocity = next_state.1;
        self.p2.angle = next_state.2;
        self.p2.velocity = next_state.3;
    }

    pub fn reset(&mut self) {
        self.p1 = self.initial_state.0.clone();
        self.p2 = self.initial_state.1.clone();
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
        } else if *t <= -PI {
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
            mass: 1.0,
        }
    }
}
