use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Robot;

#[derive(Component, Debug, Default)]
pub struct Link;

#[derive(Component, Debug, Default)]
pub struct Joint;

pub fn spawn_default_robot(commands: &mut Commands, position: Vec2) -> Entity {
    commands
        .spawn((
            Robot,
            RigidBody::Dynamic,
            Collider::circle(50.0),
            Restitution::new(0.8),
            Transform::from_xyz(position.x, position.y, 0.0),
        ))
        .id()
}

pub fn add_link(commands: &mut Commands, position: Vec2, is_base: bool) -> Entity {
    if is_base {
        commands
            .spawn((
                Link,
                RigidBody::Static,
                Collider::rectangle(20.0, 100.0),
                Restitution::ZERO,
                Transform::from_xyz(position.x, position.y, 0.0),
            ))
            .id()
    } else {
        commands
            .spawn((
                Link,
                RigidBody::Dynamic,
                Collider::rectangle(20.0, 100.0),
                Restitution::ZERO,
                Transform::from_xyz(position.x, position.y, 0.0),
            ))
            .id()
    }
}

pub fn attach_link(commands: &mut Commands, parent: Entity, child: Entity) {
    commands.spawn((
        Joint,
        RevoluteJoint::new(parent, child)
            .with_local_anchor1(Vec2::new(0.0, -50.0))
            .with_local_anchor2(Vec2::new(0.0, 50.0)),
    ));
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
