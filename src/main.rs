use double_pendulum::*;
use nannou::prelude::*;

const NUM_PENDULUMS: u32 = 1000;
const OFFSET: f32 = 0.000001;

const TIME_STEP: f32 = 0.025;
const TIME_SCALE: f32 = 0.25;
const LINE_MUL: f32 = 175.;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut pendulums = vec![];
    let mut angle = 3. * PI / 4. + NUM_PENDULUMS as f32*OFFSET;
    for i in 0..NUM_PENDULUMS {
        let l = i as f32 / NUM_PENDULUMS as f32;
        pendulums.push(DoublePendulum {
            t1: 3. * PI / 4.,
            t2: angle,
            col: hsla(2./3., 1.0, l, l),
            ..Default::default()
        });
        angle -= OFFSET;
    }
    let prev_time = 0.;

    let step_forward = false;
    let step = false;

    Model {
        // _window,
        pendulums,
        prev_time,
        step_forward,
        step,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.step || !model.step_forward {
        for pendulum in &mut model.pendulums {
            let time_step = if !model.step {
                (app.time - model.prev_time) * TIME_SCALE
            } else {
                TIME_STEP
            };
            runge_kutta_step(pendulum, time_step);
        }
        model.step = false;
        model.prev_time = app.time;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for pendulum in &model.pendulums {
        let point1_pos = vec2(
            pendulum.l1 * pendulum.t1.sin(),
            -pendulum.l1 * pendulum.t1.cos(),
        );

        draw.line()
            .start(vec2(0., 0.))
            .end(point1_pos * LINE_MUL)
            .color(pendulum.col)
            .caps_round()
            .weight(2.0);
        // draw.ellipse()
        //     .xy(point1_pos * LINE_MUL)
        //     .color(WHITE)
        //     .radius(pendulum.m2);

        let point2_pos = vec2(
            point1_pos.x + pendulum.l2 * pendulum.t2.sin(),
            point1_pos.y - pendulum.l2 * pendulum.t2.cos(),
        );

        draw.line()
            .start(point1_pos * LINE_MUL)
            .end(point2_pos * LINE_MUL)
            .color(pendulum.col)
            .caps_round()
            .weight(2.0);
        // draw.ellipse()
        //     .xy(point2_pos * LINE_MUL)
        //     .color(WHITE)
        //     .radius(pendulum.m2);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.step = true,
        Key::Return => model.step_forward = !model.step_forward,
        _ => (),
    }
}