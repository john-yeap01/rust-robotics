use avian2d::prelude::*;
use bevy::prelude::*;

#[cfg(feature = "debug_visuals")]
use avian2d::debug_render::PhysicsDebugPlugin;

use crate::robot::{add_link, attach_link, spawn_default_robot};
const PIXELS_PER_METER: f32 = 100.0;
const EARTH_GRAVITY_MPS2: f32 = 9.81;
const SCALED_GRAVITY: f32 = EARTH_GRAVITY_MPS2 * PIXELS_PER_METER;
const GROUND_HALF_WIDTH: f32 = 500.0;
const GROUND_HALF_HEIGHT: f32 = 50.0;
const HEIGHT_OFFSET: f32 = -30.0;
const GROUND_Y: f32 = -100.0+HEIGHT_OFFSET;
const DEFAULT_ROBOT_START: Vec2 = Vec2::new(15.0, 400.0);
const BASE_LINK_POS: Vec2 = Vec2::new(0.0, 0.0+HEIGHT_OFFSET);
const CHILD_LINK_POS: Vec2 = Vec2::new(0.0, 120.0+HEIGHT_OFFSET);

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    configure_simulation(&mut app);

    app
}

fn configure_simulation(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, -SCALED_GRAVITY)))
        .add_systems(Startup, setup_simulation);

    #[cfg(feature = "debug_visuals")]
    app.add_plugins(PhysicsDebugPlugin::default())
        .add_systems(Update, log_rigid_body_positions);
}

fn setup_simulation(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(GROUND_HALF_WIDTH * 2.0, GROUND_HALF_HEIGHT * 2.0),
        Restitution::ZERO,
        Transform::from_xyz(0.0, GROUND_Y, 0.0),
    ));

    spawn_default_robot(&mut commands, DEFAULT_ROBOT_START);
    let base = add_link(&mut commands, BASE_LINK_POS, true);
    let child = add_link(&mut commands, CHILD_LINK_POS, false);
    attach_link(&mut commands, base, child);
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
