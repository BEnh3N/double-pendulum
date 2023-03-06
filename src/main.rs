use double_pendulum::*;
use nannou::prelude::*;
use nannou_egui::{
    self,
    egui::{
        self,
        plot::{Line, Value, Values},
        Color32,
    },
    Egui,
};

const RAD_TO_DEG: f64 = 57.2958;
const LINE_MUL: f32 = 175.;

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

            // let new_t1 = p.t1 + p.v1 * time_step + p.a1 * (time_step * time_step * 0.5);

            // let num1 = -G * (2. * p.m1 + p.m2) * p.t1.sin();
            // let num2 = -G * p.m2 * (p.t1 - 2. * p.t2).sin();
            // let num3 = -2. * (p.t1 - p.t2).sin() * p.m2;
            // let num4 = (p.v2 * p.v2) * p.l2 + (p.v1 * p.v1) * p.l1 * (p.t1 - p.t2).cos();
            // let den = p.l1 * (2. * p.m1 + p.m2 - p.m2 * (2. * p.t1 - 2. * p.t2).cos());

            // let new_a1 = (num1 + num2 + num3 * num4) / den;

            // let new_v1 = p.v1 + (p.a1 + new_a1) * (time_step * 0.5);

            // let new_t2 = p.t2 + p.v2 * time_step + p.a2 * (time_step * time_step * 0.5);

            // let num1 = 2. * (p.t1 - p.t2).sin();
            // let num2 = (p.v1 * p.v1) * p.l1 * (p.m1 + p.m2);
            // let num3 = G * (p.m1 + p.m2) * p.t1.cos();
            // let num4 = (p.v2 * p.v2) * p.l2 * p.m2 * (p.t1 - p.t2).cos();
            // let den = p.l2 * (2. * p.m1 + p.m2 - p.m2 * (2. * p.t1 - 2. * p.t2).cos());

            // let new_a2 = num1 * (num2 + num3 + num4) / den;

            // let new_v2 = p.v2 + (p.a2 + new_a2) * (time_step * 0.5);

            // p.t1 = new_t1;
            // p.v1 = new_v1;
            // p.a1 = new_a1;

            // p.t2 = new_t2;
            // p.v2 = new_v2;
            // p.a2 = new_a2;

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
    update_ui(model);
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
        // draw.ellipse()
        //     .xy(point1_pos * LINE_MUL)
        //     .color(WHITE)
        //     .radius(pendulum.m2);

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
        // draw.ellipse()
        //     .xy(point2_pos * LINE_MUL)
        //     .color(WHITE)
        //     .radius(pendulum.m2);
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

fn update_ui(model: &mut Model) {
    let ctx = model.egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if ui.button("RESET").clicked() {
            model.pendulums = model.initial_state.clone();
            model.points.clear();
        }
        ui.horizontal(|ui| {
            ui.label("Time Rate");
            ui.add(
                egui::DragValue::new(&mut model.time_rate)
                    .clamp_range(0.1..=2.0)
                    .suffix("x")
                    .speed(0.01),
            );
        });
        ui.checkbox(&mut model.step_forward, "Step Forward");

        ui.add_enabled_ui(model.step_forward, |ui| {
            ui.horizontal(|ui| {
                ui.label("Time Step");
                ui.add(
                    egui::DragValue::new(&mut model.time_step)
                        .clamp_range(0.0001..=0.1)
                        .fixed_decimals(3)
                        .speed(0.001)
                        .suffix("s"),
                );
                ui.separator();
                if ui.button("STEP").clicked() {
                    model.step = true;
                }
            });
        });

        let mut plot = egui::plot::Plot::new("angle_plot")
            .width(120.0)
            .height(120.0)
            .view_aspect(1.0)
            .center_x_axis(true)
            .center_y_axis(true);

        for line_segment in &model.points {
            let line = Line::new(Values::from_values(line_segment.clone())).color(Color32::GOLD);
            plot = plot.line(line);
        }

        ui.add(plot);

        if ui.button("CLEAR GRAPH").clicked() {
            model.points.clear();
        }

        ui.collapsing("Pendulums", |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, pendulum) in &mut model.pendulums.iter_mut().enumerate() {
                    ui.collapsing(format!("Pendulum {i}").as_str(), |ui| {
                        // PENDULUM 1 DETAILS
                        ui.horizontal(|ui| {
                            ui.heading("P1");
                            ui.add(
                                egui::DragValue::new(&mut pendulum.l1)
                                    .clamp_range(0.1..=1.5)
                                    .speed(0.01)
                                    .suffix("m"),
                            );
                            ui.add(
                                egui::DragValue::new(&mut pendulum.m1)
                                    .clamp_range(0.1..=5.0)
                                    .speed(0.05)
                                    .suffix("kg"),
                            );
                        });
                        ui.label(format!("{:.5}°", pendulum.t1 * RAD_TO_DEG));
                        ui.label(format!("{:.5} m/s", pendulum.v1));

                        // PENDULUM 2 DETAILS
                        ui.horizontal(|ui| {
                            ui.heading("P2");
                            ui.add(
                                egui::DragValue::new(&mut pendulum.l2)
                                    .clamp_range(0.1..=1.5)
                                    .speed(0.01)
                                    .suffix("m"),
                            );
                            ui.add(
                                egui::DragValue::new(&mut pendulum.m2)
                                    .clamp_range(0.1..=5.0)
                                    .speed(0.05)
                                    .suffix("kg"),
                            );
                        });
                        ui.label(format!("{:.5}°", pendulum.t2 * RAD_TO_DEG));
                        ui.label(format!("{:.5} m/s", pendulum.v2));

                        ui.color_edit_button_hsva(&mut pendulum.col);
                    });
                }
            });
        });
    });
}
