use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
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
    // initialize_pendulums(&mut commands, 1, PI / 6.0, 0.0, 0.0);
    initialize_pendulums(&mut commands, 1000, PI05, 0.000001, 2.0 / 3.0);
}

fn update(
    time: Res<Time>,
    time_rate: Res<TimeRate>,
    mut pendulums: Query<&mut DoublePendulum>,
    mut points: ResMut<Points>,
    mut step_forward: ResMut<StepForward>,
) {
    if !step_forward.enabled || step_forward.step {
        let t = match step_forward.enabled {
            true => step_forward.time_step,
            false => time.delta_secs_f64() * time_rate.0,
        };

        for (i, mut pendulum) in pendulums.iter_mut().enumerate() {
            runge_kutta_step(&mut pendulum, t);

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
    }
}

fn key_press(
    keys: Res<ButtonInput<KeyCode>>,
    mut pendulums: Query<&mut DoublePendulum>,
    mut points: ResMut<Points>,
    mut step_forward: ResMut<StepForward>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        pendulums.iter_mut().for_each(|mut p| p.reset());
        points.empty();
    }
    if keys.just_pressed(KeyCode::ShiftLeft) {
        step_forward.enabled = !step_forward.enabled;
    }
    if step_forward.enabled && keys.just_pressed(KeyCode::Space) {
        step_forward.step = true;
    }
}
