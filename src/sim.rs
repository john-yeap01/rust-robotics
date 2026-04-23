use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[cfg(feature = "debug_visuals")]
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::control::ControlPlugin;
use crate::robot::{add_link, attach_base_to_hub, attach_link_to_hub, spawn_joint_hub};

const PIXELS_PER_METER: f32 = 100.0;
const GROUND_HALF_WIDTH: f32 = 500.0;
const GROUND_HALF_HEIGHT: f32 = 50.0;
const HEIGHT_OFFSET: f32 = -30.0;
const GROUND_Y: f32 = -100.0 + HEIGHT_OFFSET;
const BASE_LINK_POS: Vec2 = Vec2::new(0.0, 0.0 + HEIGHT_OFFSET);
const LINK_HALF_HEIGHT: f32 = 50.0;
const HUB_RADIUS: f32 = 5.0;
const HUB_POS: Vec2 = Vec2::new(0.0, BASE_LINK_POS.y + LINK_HALF_HEIGHT + HUB_RADIUS);
const CHILD_LINK_POS: Vec2 = Vec2::new(0.0, HUB_POS.y + HUB_RADIUS + LINK_HALF_HEIGHT);

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, ControlPlugin));
    configure_simulation(&mut app);

    app
}

fn configure_simulation(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ))
    .insert_resource(TimestepMode::Fixed {
        dt: 1.0 / 60.0,
        substeps: 12,
    })
    .add_systems(Startup, setup_simulation);

    #[cfg(feature = "debug_visuals")]
    app.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, log_rigid_body_positions);
}

fn setup_simulation(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(GROUND_HALF_WIDTH, GROUND_HALF_HEIGHT),
        Restitution::coefficient(0.0),
        Transform::from_xyz(0.0, GROUND_Y, 0.0),
    ));

    let base = add_link(&mut commands, BASE_LINK_POS, true);
    let child = add_link(&mut commands, CHILD_LINK_POS, false);
    let hub = spawn_joint_hub(&mut commands, HUB_POS, HUB_RADIUS);
    attach_base_to_hub(
        &mut commands,
        base,
        hub,
        Vec2::new(0.0, LINK_HALF_HEIGHT),
        Vec2::new(0.0, -HUB_RADIUS),
    );
    attach_link_to_hub(
        &mut commands,
        child,
        hub,
        Vec2::new(0.0, -LINK_HALF_HEIGHT),
        Vec2::new(0.0, HUB_RADIUS),
    );
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
    use crate::control::JointTarget;

    #[test]
    fn configure_simulation_adds_core_resources() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, ControlPlugin));
        configure_simulation(&mut app);

        assert!(app.world().contains_resource::<JointTarget>());
        assert!(app.world().contains_resource::<TimestepMode>());
    }
}
