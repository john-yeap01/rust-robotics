use avian2d::prelude::*;
use bevy::prelude::*;

#[cfg(feature = "debug_visuals")]
use avian2d::debug_render::PhysicsDebugPlugin;

const PIXELS_PER_METER: f32 = 100.0;
const GRAVITY_Y: f32 = -9.81 * PIXELS_PER_METER;

const FLOOR_SIZE: Vec2 = Vec2::new(900.0, 40.0);
const FLOOR_Y: f32 = -220.0;

const HORIZONTAL_RECT_SIZE: Vec2 = Vec2::new(220.0, 30.0);
const HORIZONTAL_RECT_START: Vec2 = Vec2::new(0.0, -80.0);

const VERTICAL_RECT_SIZE: Vec2 = Vec2::new(30.0, 220.0);
const VERTICAL_RECT_START: Vec2 = Vec2::new(0.0, 180.0);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Avian Rectangle Drop".into(),
            resolution: (1100, 800).into(),
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
        Sprite::from_color(Color::srgb(0.28, 0.31, 0.36), FLOOR_SIZE),
        Transform::from_xyz(0.0, FLOOR_Y, 0.0),
        RigidBody::Static,
        Collider::rectangle(FLOOR_SIZE.x, FLOOR_SIZE.y),
        Friction::new(0.9),
        Restitution::ZERO,
        CollisionEventsEnabled,
    ));

    spawn_box(
        &mut commands,
        "Horizontal Rectangle",
        HORIZONTAL_RECT_SIZE,
        HORIZONTAL_RECT_START,
        Color::srgb(0.18, 0.55, 0.82),
    );

    spawn_box(
        &mut commands,
        "Vertical Rectangle",
        VERTICAL_RECT_SIZE,
        VERTICAL_RECT_START,
        Color::srgb(0.91, 0.46, 0.24),
    );
}

fn spawn_box(
    commands: &mut Commands,
    name: &'static str,
    size: Vec2,
    position: Vec2,
    color: Color,
) {
    commands.spawn((
        Name::new(name),
        Sprite::from_color(color, size),
        Transform::from_xyz(position.x, position.y, 1.0),
        RigidBody::Dynamic,
        Collider::rectangle(size.x, size.y),
        Friction::new(0.8),
        Restitution::new(0.05),
        LinearDamping(0.05),
        AngularDamping(0.2),
        CollisionEventsEnabled,
    ));
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
