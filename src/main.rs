use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui::Color32, EguiPlugin, EguiPrimaryContextPass};
use bevy_vector_shapes::prelude::*;
use double_pendulum::{dp::*, ui::*, *};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Shape2dPlugin::default(),
            EguiPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Gravity(9.81))
        .init_resource::<TimeRate>()
        .init_resource::<Points>()
        .init_resource::<StepForward>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw, update, key_press))
        .add_systems(EguiPrimaryContextPass, ui_update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn(DoublePendulum::new(2.49, 0.25, Color32::WHITE));
    // commands.spawn(DoublePendulum::new(PI, PI, Color32::WHITE));
    // initialize_pendulums(&mut commands, 1000, PI05, 0.000001, 2.0 / 3.0);
}

fn update(
    gravity: Res<Gravity>,
    mut pendulums: Query<&mut DoublePendulum>,
    mut points: ResMut<Points>,
    mut step_forward: ResMut<StepForward>,
    time: Res<Time>,
    time_rate: Res<TimeRate>,
) {
    if !step_forward.enabled || step_forward.step {
        let time_step = match step_forward.enabled {
            true => step_forward.time_step,
            false => time.delta_secs_f64() * time_rate.0,
        };

        for (i, mut pendulum) in pendulums.iter_mut().enumerate() {
            pendulum.step(time_step, gravity.0);

            if (pendulum.p1.clamp() || pendulum.p2.clamp()) && i == 0 {
                points.add_line();
            }

            if i == 0 {
                points.push([pendulum.p1.angle, pendulum.p2.angle]);
            }
        }

        step_forward.step = false
    }
}

fn draw(mut painter: ShapePainter, pendulums: Query<&DoublePendulum>) {
    painter.thickness = 2.0;
    for pendulum in pendulums {
        let c = pendulum.col;
        painter.color = Color::hsva(c.h * 360.0, c.s, c.v, c.a);

        let p1 = &pendulum.p1;
        let p1_pos = vec3(
            (p1.length * p1.angle.sin()) as f32,
            (p1.length * -p1.angle.cos()) as f32,
            0.0,
        );
        painter.line(Vec3::ZERO, p1_pos * LINE_MUL);

        let p2 = &pendulum.p2;
        let p2_pos = vec3(
            (p2.length * p2.angle.sin()) as f32,
            (p2.length * -p2.angle.cos()) as f32,
            0.0,
        );
        painter.line(p1_pos * LINE_MUL, (p1_pos + p2_pos) * LINE_MUL);

        painter.translate(p1_pos * LINE_MUL);
        painter.circle(p1.mass.sqrt() as f32 * 4.0);
        painter.translate(p2_pos * LINE_MUL);
        painter.circle(p2.mass.sqrt() as f32 * 4.0);
    }
}
