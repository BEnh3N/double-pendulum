use double_pendulum::*;
use nannou::prelude::*;
use nannou_egui::{
    self,
    egui::{
        self,
        plot::{Line, Value, Values},
    },
    Egui,
};

// const NUM_PENDULUMS: u32 = 1000;
// const OFFSET: f64 = 0.000001;

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

    // let pendulums = initialize_pendulums(1, PI_F64 / 2.0, 0.000001, 2./3.);
    let pendulums = vec![DoublePendulum {
        t1: PI_F64 / 6.,
        ..Default::default()
    }];

    let time_rate = 1.0;
    let time_step = 0.025;
    let g = 9.81;

    let step_forward = false;
    let step = false;

    let points = vec![];

    Model {
        // _window,
        egui,
        pendulums,
        time_rate,
        time_step,
        g,
        step_forward,
        step,
        points,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.step || !model.step_forward {
        for pendulum in &mut model.pendulums {
            let time_step = if !model.step {
                update.since_last.as_secs_f64() * model.time_rate
            } else {
                model.time_step
            };

            pendulum.t1 = limit_angle(pendulum.t1);
            pendulum.t2 = limit_angle(pendulum.t2);
            runge_kutta_step(pendulum, time_step);
        }
        model.step = false;
    }

    model.points.push(Value {
        x: model.pendulums[0].t1,
        y: model.pendulums[0].t2,
    });

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
        _ => (),
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
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

        let line = Line::new(Values::from_values(model.points.clone()));
        ui.add(
            egui::plot::Plot::new("dev_plot")
                .line(line)
                // .data_aspect(1.0)
                .width(120.0)
                .height(120.0)
                .view_aspect(1.0)
                .center_x_axis(true)
                .center_y_axis(true),
        );

        ui.collapsing("Pendulums", |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, pendulum) in &mut model.pendulums.iter_mut().enumerate() {
                    ui.collapsing(format!("Pendulum {}", i).as_str(), |ui| {
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
