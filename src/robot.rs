use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Robot;

pub fn spawn_default_robot(commands: &mut Commands, position: Vec2) -> Entity {
    commands
        .spawn((
            Robot,
            RigidBody::Dynamic,
            Collider::ball(50.0),
            Restitution::coefficient(1.2),
            Transform::from_xyz(position.x, position.y, 0.0),
        ))
        .id()
}

// pub fn add_joint(commands: &mut Commands, position)

pub fn add_link(commands: &mut Commands, position: Vec2) {

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
