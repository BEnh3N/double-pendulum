use std::f64::consts::PI;

use nannou_egui::egui::epaint::Hsva;

pub const NUM_ARGS: usize = 7;

#[derive(Clone)]
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

    pub col: Hsva,
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
            t1: PI / 2.,
            v1: 0.,
            t2: 0.,
            v2: 0.,
            a1: 0.,
            a2: 0.,

            t: 0.,

            m1: 2.,
            m2: 2.,
            l1: 1.,
            l2: 1.,
            col: Hsva::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}
