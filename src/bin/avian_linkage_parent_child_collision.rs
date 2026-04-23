use avian2d::prelude::*;
use bevy::prelude::*;

#[cfg(feature = "debug_visuals")]
use avian2d::debug_render::PhysicsDebugPlugin;

const PIXELS_PER_METER: f32 = 100.0;
const GRAVITY_Y: f32 = -9.81 * PIXELS_PER_METER;

const FLOOR_SIZE: Vec2 = Vec2::new(1200.0, 36.0);
const FLOOR_Y: f32 = -280.0;

const HUB_RADIUS: f32 = 12.0;
const PARENT_SIZE: Vec2 = Vec2::new(46.0, 200.0);
const CHILD_SIZE: Vec2 = Vec2::new(34.0, 240.0);
const CHILD_START_ANGLE: f32 = -0.2;

const FLOOR_TOP_Y: f32 = FLOOR_Y + FLOOR_SIZE.y * 0.5;
const PARENT_CENTER: Vec2 = Vec2::new(0.0, FLOOR_TOP_Y + PARENT_SIZE.y * 0.5);
const HUB_POS: Vec2 = Vec2::new(0.0, FLOOR_TOP_Y + PARENT_SIZE.y + HUB_RADIUS);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Avian Parent Child Link Collision".into(),
            resolution: (1280, 900).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER))
    .insert_resource(ClearColor(Color::srgb(0.96, 0.97, 0.99)))
    .insert_resource(Gravity(Vec2::new(0.0, GRAVITY_Y)))
    .insert_resource(SubstepCount(16))
    .add_systems(Startup, setup)
    .add_systems(Update, log_collision_starts);

    #[cfg(feature = "debug_visuals")]
    app.add_plugins(PhysicsDebugPlugin::default());

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let floor = commands
        .spawn((
            Name::new("Floor"),
            Sprite::from_color(Color::srgb(0.28, 0.31, 0.36), FLOOR_SIZE),
            Transform::from_xyz(0.0, FLOOR_Y, 0.0),
            RigidBody::Static,
            Collider::rectangle(FLOOR_SIZE.x, FLOOR_SIZE.y),
            Friction::new(0.9),
            Restitution::ZERO,
            CollisionEventsEnabled,
        ))
        .id();

    let hub = commands
        .spawn((
            Name::new("Hub"),
            Mesh2d(meshes.add(Circle::new(HUB_RADIUS))),
            MeshMaterial2d(materials.add(Color::srgb(0.16, 0.18, 0.22))),
            Transform::from_xyz(HUB_POS.x, HUB_POS.y, 2.0),
            RigidBody::Dynamic,
            Collider::circle(HUB_RADIUS),
            LinearVelocity(Vec2::ZERO),
            AngularVelocity(0.0),
            Friction::new(0.8),
            Restitution::ZERO,
            LinearDamping(0.02),
            AngularDamping(0.04),
            CollisionEventsEnabled,
        ))
        .id();

    let parent = commands
        .spawn((
            Name::new("Parent Link"),
            Sprite::from_color(Color::srgb(0.88, 0.46, 0.22), PARENT_SIZE),
            Transform::from_xyz(PARENT_CENTER.x, PARENT_CENTER.y, 1.0),
            RigidBody::Dynamic,
            Collider::rectangle(PARENT_SIZE.x, PARENT_SIZE.y),
            LinearVelocity(Vec2::ZERO),
            AngularVelocity(0.0),
            Friction::new(0.9),
            Restitution::ZERO,
            LinearDamping(0.02),
            AngularDamping(0.04),
            CollisionEventsEnabled,
        ))
        .id();

    let child_center = child_center_from_hub(HUB_POS, CHILD_SIZE.y * 0.5, CHILD_START_ANGLE);
    let child = commands
        .spawn((
            Name::new("Child Link"),
            Sprite::from_color(Color::srgb(0.18, 0.55, 0.82), CHILD_SIZE),
            Transform::from_xyz(child_center.x, child_center.y, 1.0)
                .with_rotation(Quat::from_rotation_z(CHILD_START_ANGLE)),
            RigidBody::Dynamic,
            Collider::rectangle(CHILD_SIZE.x, CHILD_SIZE.y),
            LinearVelocity(Vec2::ZERO),
            AngularVelocity(0.0),
            Friction::new(0.8),
            Restitution::new(0.02),
            LinearDamping(0.01),
            AngularDamping(0.02),
            CollisionEventsEnabled,
        ))
        .id();

    commands.spawn(
        FixedJoint::new(floor, parent)
            .with_local_anchor1(Vec2::new(PARENT_CENTER.x, FLOOR_SIZE.y * 0.5))
            .with_local_anchor2(Vec2::new(0.0, -PARENT_SIZE.y * 0.5)),
    );

    commands.spawn(
        FixedJoint::new(parent, hub)
            .with_local_anchor1(Vec2::new(0.0, PARENT_SIZE.y * 0.5))
            .with_local_anchor2(Vec2::new(0.0, -HUB_RADIUS)),
    );

    commands.spawn(
        RevoluteJoint::new(hub, child)
            .with_local_anchor1(Vec2::new(0.0, HUB_RADIUS))
            .with_local_anchor2(Vec2::new(0.0, -CHILD_SIZE.y * 0.5)),
    );
}

fn child_center_from_hub(hub: Vec2, half_length: f32, angle: f32) -> Vec2 {
    let attachment_point = hub + Vec2::new(0.0, HUB_RADIUS);
    let offset = Quat::from_rotation_z(angle).mul_vec3(Vec3::new(0.0, half_length, 0.0));
    attachment_point + offset.truncate()
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
