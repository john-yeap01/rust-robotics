use avian2d::prelude::*;
use bevy::prelude::*;

#[cfg(feature = "debug_visuals")]
use avian2d::debug_render::PhysicsDebugPlugin;

const PIXELS_PER_METER: f32 = 100.0;
const GRAVITY_Y: f32 = -9.81 * PIXELS_PER_METER;

const FLOOR_SIZE: Vec2 = Vec2::new(1100.0, 40.0);
const FLOOR_Y: f32 = -260.0;

const BLOCKER_SIZE: Vec2 = Vec2::new(36.0, 220.0);
const BLOCKER_POS: Vec2 = Vec2::new(-40.0, -130.0);

const HINGE_POS: Vec2 = Vec2::new(-170.0, 150.0);
const BAR_SIZE: Vec2 = Vec2::new(28.0, 240.0);
const BAR_START_ANGLE: f32 = -1.15;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Avian Hinged Rectangle Collision".into(),
            resolution: (1200, 850).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER))
    .insert_resource(ClearColor(Color::srgb(0.96, 0.97, 0.99)))
    .insert_resource(Gravity(Vec2::new(0.0, GRAVITY_Y)))
    .insert_resource(SubstepCount(12))
    .add_systems(Startup, setup)
    .add_systems(Update, log_collision_starts);

    #[cfg(feature = "debug_visuals")]
    app.add_plugins(PhysicsDebugPlugin::default());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Name::new("Floor"),
        Sprite::from_color(Color::srgb(0.27, 0.30, 0.35), FLOOR_SIZE),
        Transform::from_xyz(0.0, FLOOR_Y, 0.0),
        RigidBody::Static,
        Collider::rectangle(FLOOR_SIZE.x, FLOOR_SIZE.y),
        Friction::new(0.9),
        Restitution::ZERO,
        CollisionEventsEnabled,
    ));

    commands.spawn((
        Name::new("Rigid Vertical Rectangle"),
        Sprite::from_color(Color::srgb(0.90, 0.45, 0.22), BLOCKER_SIZE),
        Transform::from_xyz(BLOCKER_POS.x, BLOCKER_POS.y, 1.0),
        RigidBody::Static,
        Collider::rectangle(BLOCKER_SIZE.x, BLOCKER_SIZE.y),
        Friction::new(0.8),
        Restitution::ZERO,
        CollisionEventsEnabled,
    ));

    let hinge = commands
        .spawn((
            Name::new("Hinge"),
            Sprite::from_color(Color::srgb(0.15, 0.17, 0.20), Vec2::splat(16.0)),
            Transform::from_xyz(HINGE_POS.x, HINGE_POS.y, 2.0),
            RigidBody::Static,
        ))
        .id();

    let bar_center = center_from_hinge(HINGE_POS, BAR_SIZE.y * 0.5, BAR_START_ANGLE);
    let swinging_bar = commands
        .spawn((
            Name::new("Hinged Rectangle"),
            Sprite::from_color(Color::srgb(0.18, 0.55, 0.82), BAR_SIZE),
            Transform::from_xyz(bar_center.x, bar_center.y, 1.0)
                .with_rotation(Quat::from_rotation_z(BAR_START_ANGLE)),
            RigidBody::Dynamic,
            Collider::rectangle(BAR_SIZE.x, BAR_SIZE.y),
            Friction::new(0.8),
            Restitution::new(0.02),
            LinearDamping(0.02),
            AngularDamping(0.02),
            CollisionEventsEnabled,
        ))
        .id();

    commands.spawn(
        RevoluteJoint::new(hinge, swinging_bar)
            .with_local_anchor2(Vec2::new(0.0, BAR_SIZE.y * 0.5)),
    );
}

fn center_from_hinge(hinge: Vec2, half_length: f32, angle: f32) -> Vec2 {
    let offset = Quat::from_rotation_z(angle).mul_vec3(Vec3::new(0.0, -half_length, 0.0));
    hinge + offset.truncate()
}

fn log_collision_starts(mut collisions: MessageReader<CollisionStart>, names: Query<&Name>) {
    for collision in collisions.read() {
        let first = names
            .get(collision.collider1)
            .map(Name::as_str)
            .unwrap_or("Unnamed");
        let second = names
            .get(collision.collider2)
            .map(Name::as_str)
            .unwrap_or("Unnamed");

        println!("{first} hit {second}");
    }
}
