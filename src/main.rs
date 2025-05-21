use bevy::prelude::*;

const PADDLE_SIZE: bevy::prelude::Vec2 = bevy::prelude::Vec2::new(20.0, 120.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
const WALL_PADDING: f32 = 10.0;

const WALL_THICKNESS: f32 = 10.0;

const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const RIGHT_WALL: f32 = -300.;
const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
const PADDLE_COLOR: Color = Color::srgb(0., 1.0, 0.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (apply_velocity, move_paddle).chain())
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Wall;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    let paddle_x: f32 = RIGHT_WALL + WALL_PADDING;

    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(paddle_x, paddle_y, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle,
    ));
}


fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }

    let new_paddle_position =
        paddle_transform.translation.y + direction * PADDLE_SPEED * time.delta_secs();

    let top_bound = TOP_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + WALL_PADDING;
    let bottom_bound = BOTTOM_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - WALL_PADDING;

    paddle_transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}
