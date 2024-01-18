use nannou_egui::egui::{
    self,
    plot::{Line, Values},
    Color32,
};

use crate::{Model, RAD_TO_DEG};

pub fn update_ui(model: &mut Model) {
    let ctx = model.egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if ui.button("RESET").clicked() {
            model.pendulums = model.initial_state.clone();
            model.points = vec![vec![]];
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
