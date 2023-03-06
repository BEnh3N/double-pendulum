use double_pendulum::*;
use nannou::prelude::*;
use nannou_egui::{self, egui::plot::Value, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .key_pressed(key_pressed)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let egui = Egui::from_window(&app.window(window_id).unwrap());

    // let pendulums = initialize_pendulums(1000, PI_F64 / 2., 0.000001, 2. / 3.);
    let pendulums = initialize_pendulums(1, PI_F64 / 6., PI_F64, 1.);

    let limit_angles = true;

    let time_rate = 1.0;
    let time_step = 0.025;
    let g = 9.81;

    let step_forward = false;
    let step = false;

    let points = vec![vec![]];

    let initial_state = pendulums.clone();

    Model {
        // _window,
        egui,
        pendulums,
        limit_angles,
        time_rate,
        time_step,
        g,
        step_forward,
        step,
        points,
        initial_state,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.step || !model.step_forward {
        let last_i = model.pendulums.len() - 1;
        let mut skip_line = false;

        for (i, p) in &mut model.pendulums.iter_mut().enumerate() {
            let time_step = if !model.step {
                update.since_last.as_secs_f64() * model.time_rate
            } else {
                model.time_step
            };

            runge_kutta_step(p, time_step);

            if model.limit_angles {
                let (limit1, limit2);
                (p.t1, limit1) = limit_angle(p.t1);
                (p.t2, limit2) = limit_angle(p.t2);

                if (limit1 || limit2) && i == last_i {
                    skip_line = true
                }
            }
        }

        if skip_line {
            model.points.push(vec![]);
        }

        let i = model.points.len();
        model.points[i - 1].push(Value {
            x: model.pendulums[last_i].t1,
            y: model.pendulums[last_i].t2,
        });

        model.step = false;
    }

    model.egui.set_elapsed_time(update.since_start);
    ui::update_ui(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for pendulum in &model.pendulums {
        let point1_pos = vec2(
            (pendulum.l1 * pendulum.t1.sin()) as f32,
            (-pendulum.l1 * pendulum.t1.cos()) as f32,
        );

        let h = pendulum.col.h;
        let s = pendulum.col.s;
        let v = pendulum.col.v;
        let a = pendulum.col.a;

        draw.line()
            .start(vec2(0., 0.))
            .end(point1_pos * LINE_MUL)
            .color(hsva(h, s, v, a))
            .caps_round()
            .weight(2.0);

        let point2_pos = vec2(
            point1_pos.x + (pendulum.l2 * pendulum.t2.sin()) as f32,
            point1_pos.y - (pendulum.l2 * pendulum.t2.cos()) as f32,
        );

        draw.line()
            .start(point1_pos * LINE_MUL)
            .end(point2_pos * LINE_MUL)
            .color(hsva(h, s, v, a))
            .caps_round()
            .weight(2.0);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            if model.step_forward {
                model.step = true
            }
        }
        Key::LShift => model.step_forward = !model.step_forward,
        Key::R => {
            model.pendulums = model.initial_state.clone();
            model.points.clear();
        }
        _ => (),
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
