use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::robot::spawn_default_robot;

const PIXELS_PER_METER: f32 = 100.0;
const GROUND_HALF_WIDTH: f32 = 500.0;
const GROUND_HALF_HEIGHT: f32 = 50.0;
const GROUND_Y: f32 = -100.0;
const DEFAULT_ROBOT_START: Vec2 = Vec2::new(0.0, 400.0);

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    configure_simulation(&mut app);

    app
}

fn configure_simulation(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ))
    .add_systems(Startup, setup_simulation);

    #[cfg(feature = "debug_visuals")]
    app.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, log_rigid_body_positions);
}

fn setup_simulation(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Collider::cuboid(GROUND_HALF_WIDTH, GROUND_HALF_HEIGHT),
        Transform::from_xyz(0.0, GROUND_Y, 0.0),
    ));

    spawn_default_robot(&mut commands, DEFAULT_ROBOT_START);
}

#[cfg(feature = "debug_visuals")]
fn log_rigid_body_positions(positions: Query<&Transform, With<RigidBody>>) {
    for transform in &positions {
        println!("Position: {:?}", transform.translation);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::robot::Robot;

    fn build_headless_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, setup_simulation);
        app
    }

    #[test]
    fn build_app_includes_simulation_entities_after_startup() {
        let mut app = build_headless_app();
        app.update();

        let mut robot_query = app.world_mut().query::<&Robot>();
        let robot_count = robot_query.iter(app.world()).count();
        assert_eq!(robot_count, 1);
    }
}
