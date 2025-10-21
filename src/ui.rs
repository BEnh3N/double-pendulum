use std::f64::consts::PI;

use crate::{
    dp::{DoublePendulum, Pendulum},
    Gravity, Points, StepForward, TimeRate, RAD_TO_DEG,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, text::LayoutJob, Color32, DragValue, TextFormat, Ui, Vec2b},
    EguiContexts,
};
use egui_plot::{Line, Plot};

pub fn ui_update(
    mut contexts: EguiContexts,
    mut gravity: ResMut<Gravity>,
    mut pendulums: Query<&mut DoublePendulum>,
    mut points: ResMut<Points>,
    mut step_forward: ResMut<StepForward>,
    mut time_rate: ResMut<TimeRate>,
) {
    let ctx = contexts.ctx_mut().unwrap();
    egui::SidePanel::left("Settings")
        .resizable(false)
        .show(ctx, |ui| {
            if ui.button("RESET").clicked() {
                pendulums.iter_mut().for_each(|mut p| p.reset());
                points.empty();
            }
            ui.horizontal(|ui| {
                ui.label("Time Rate");
                ui.add(
                    DragValue::new(&mut time_rate.0)
                        .range(0.1..=2.0)
                        .suffix("x")
                        .speed(0.01),
                );
            });

            ui.checkbox(&mut step_forward.enabled, "Step Forward");
            ui.add_enabled_ui(step_forward.enabled, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Time Step");
                    ui.add(
                        DragValue::new(&mut step_forward.time_step)
                            .range(0.0001..=0.1)
                            .fixed_decimals(3)
                            .speed(0.001)
                            .suffix("s"),
                    );
                    ui.separator();
                    if ui.button("STEP").clicked() {
                        step_forward.step = true;
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Gravity");
                ui.add(
                    DragValue::new(&mut gravity.0)
                        .range(0.0..=1000.0)
                        .speed(0.01)
                        .suffix("m/s²"),
                );
            });

            // Angle plot
            ui.separator();
            ui.horizontal(|ui| {
                ui.heading("Angle Plot");
                if ui.button("🗑").clicked() {
                    points.empty();
                }
            });
            Plot::new("angle_plot")
                .width(ui.available_width())
                .auto_bounds(false)
                .view_aspect(1.0)
                .default_x_bounds(-PI, PI)
                .default_y_bounds(-PI, PI)
                .show(ui, |plot_ui| {
                    for line_segment in points.0.clone() {
                        let line = Line::new("segment", line_segment).color(Color32::GOLD);
                        plot_ui.line(line);
                    }
                });

            ui.separator();
            pendulum_editor(ui, pendulums);
        });
}

fn pendulum_editor(ui: &mut Ui, mut pendulums: Query<&mut DoublePendulum>) {
    ui.heading("Pendulums");
    egui::ScrollArea::vertical()
        .auto_shrink(Vec2b::new(false, false))
        .show(ui, |ui| {
            for (i, mut pendulum) in pendulums.iter_mut().enumerate() {
                let mut pendulum_text = LayoutJob::default();
                pendulum_text.append(
                    "⏺",
                    0.0,
                    TextFormat {
                        color: pendulum.col.into(),
                        ..Default::default()
                    },
                );
                pendulum_text.append(
                    format!(" Pendulum {}", i).as_str(),
                    0.0,
                    TextFormat::default(),
                );

                ui.collapsing(pendulum_text, |ui| {
                    pendulum_details(ui, &mut pendulum.p1, "Arm 1");
                    pendulum_details(ui, &mut pendulum.p2, "Arm 2");

                    ui.color_edit_button_hsva(&mut pendulum.col);

                    if ui.button("Reset").clicked() {
                        pendulum.reset();
                    }
                });
            }
        });
}

fn pendulum_details(ui: &mut Ui, pendulum: &mut Pendulum, name: &str) {
    ui.heading(name);
    ui.horizontal(|ui| {
        ui.add(
            egui::DragValue::new(&mut pendulum.length)
                .range(0.1..=1.5)
                .speed(0.01)
                .suffix("m"),
        );
        ui.add(
            egui::DragValue::new(&mut pendulum.mass)
                .range(0.1..=5.0)
                .speed(0.05)
                .suffix("kg"),
        );
    });
    ui.label(format!("{:.5}°", pendulum.angle * RAD_TO_DEG));
    ui.label(format!("{:.5} rad/s", pendulum.velocity));
}
