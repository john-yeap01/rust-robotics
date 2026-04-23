use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::control::ActuatedJoint;

#[derive(Component, Debug, Default)]
pub struct Robot;

#[derive(Component, Debug, Default)]
pub struct Link;

#[derive(Component, Debug, Default)]
pub struct Joint;

const CHILD_START_ANGLE_RADIANS: f32 = 0.2;

pub fn spawn_default_robot(commands: &mut Commands, position: Vec2) -> Entity {
    commands
        .spawn((
            Robot,
            RigidBody::Dynamic,
            Collider::ball(50.0),
            Restitution::coefficient(0.8),
            Transform::from_xyz(position.x, position.y, 0.0),
        ))
        .id()
}

pub fn add_link(commands: &mut Commands, position: Vec2, is_base: bool) -> Entity {
    if is_base {
        commands
            .spawn((
                Link,
                RigidBody::Fixed,
                Collider::cuboid(10.0, 50.0),
                Restitution::coefficient(0.0),
                Transform::from_xyz(position.x, position.y, 0.0),
            ))
            .id()
    } else {
        commands
            .spawn((
                Link,
                RigidBody::Dynamic,
                Collider::cuboid(10.0, 50.0),
                Restitution::coefficient(0.0),
                Transform::from_xyz(position.x, position.y, 0.0)
                    .with_rotation(Quat::from_rotation_z(CHILD_START_ANGLE_RADIANS)),
            ))
            .id()
    }
}

pub fn spawn_joint_hub(commands: &mut Commands, position: Vec2, radius: f32) -> Entity {
    commands
        .spawn((
            Joint,
            RigidBody::Dynamic,
            Collider::ball(radius),
            Restitution::coefficient(0.0),
            Transform::from_xyz(position.x, position.y, 0.0),
        ))
        .id()
}

pub fn attach_base_to_hub(
    commands: &mut Commands,
    base: Entity,
    hub: Entity,
    base_anchor: Vec2,
    hub_anchor: Vec2,
) {
    let joint = FixedJointBuilder::new()
        .local_anchor1(base_anchor)
        .local_anchor2(hub_anchor);

    commands.entity(hub).insert(ImpulseJoint::new(base, joint));
}

pub fn attach_link_to_hub(
    commands: &mut Commands,
    child: Entity,
    hub: Entity,
    child_anchor: Vec2,
    hub_anchor: Vec2,
) {
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(hub_anchor)
        .local_anchor2(child_anchor)
        .motor_model(MotorModel::AccelerationBased)
        .motor_position(2.0, 80.0, 12.0)
        .motor_max_force(500.0);

    commands
        .entity(child)
        .insert((ActuatedJoint, ImpulseJoint::new(hub, joint)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn robot_marker_is_default_constructible() {
        let robot = Robot::default();
        assert_eq!(format!("{robot:?}"), "Robot");
    }

    #[test]
    fn spawn_helper_adds_robot_component() {
        let mut app = App::new();

        let entity = app.world_mut().run_system_once(|mut commands: Commands| {
            spawn_default_robot(&mut commands, Vec2::new(1.0, 2.0))
        });
        let entity = entity.expect("spawn system should return the robot entity");

        assert!(app.world().entity(entity).contains::<Robot>());
        assert!(app.world().entity(entity).contains::<RigidBody>());
        assert!(app.world().entity(entity).contains::<Collider>());
    }
}
