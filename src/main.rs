use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 480.0;
pub const PLAYER_SIZE: f32 = 100.0;
pub const ROCK_COOLDOWN: f32 = 2.0;
pub const FAST_ROCK_SPEED: f32 = 100.0;
pub const NORMAL_ROCK_SPEED: f32 = 75.0;
pub const SLOW_ROCK_SPEED: f32 = 50.0;
pub const BIG_ROCK_SIZE: f32 = 150.0;
pub const NORMAL_ROCK_SIZE: f32 = 100.0;
pub const SMALL_ROCK_SIZE: f32 = 70.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.2)))
        .init_resource::<RockSpawnTimer>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space game".into(),
                resolution: (900.0, 900.0).into(),
                resizable: false,

                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(spawn_rocks_over_time)
        .add_system(tick_rock_spawn_timer)
        .add_system(player_movement)
        .add_system(move_rocks)
        .add_system(remove_rocks)
        .run();
}

#[derive(Resource)]
pub struct RockSpawnTimer {
    pub timer: Timer,
}

impl Default for RockSpawnTimer {
    fn default() -> Self {
        RockSpawnTimer {
            timer: Timer::from_seconds(ROCK_COOLDOWN, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Player {}

enum RockSize {
    Big,
    Normal,
    Small,
}

enum RockSpeed {
    Fast,
    Normal,
    Slow,
}

#[derive(Component)]
pub struct Rock {
    size: RockSize,
    speed: RockSpeed,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/character.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut new_translation =
            transform.translation + direction * PLAYER_SPEED * time.delta_seconds();

        if new_translation.x < x_min {
            new_translation.x = x_min;
        } else if new_translation.x > x_max {
            new_translation.x = x_max;
        }

        if new_translation.y < y_min {
            new_translation.y = y_min;
        } else if new_translation.y > y_max {
            new_translation.y = y_max;
        }

        transform.translation = new_translation;
    }
}

pub fn spawn_rocks_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    rock_spawn_timer: Res<RockSpawnTimer>,
) {
    if rock_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let mut rng = thread_rng();
        let (rock_size, rock_sprite) = match rng.gen_range(0..3) {
            0 => (RockSize::Small, "sprites/small_rock.png"),
            1 => (RockSize::Normal, "sprites/normal_rock.png"),
            _ => (RockSize::Big, "sprites/big_rock.png"),
        };

        let rock_speed: RockSpeed = match rng.gen_range(0..3) {
            0 => RockSpeed::Fast,
            1 => RockSpeed::Normal,
            _ => RockSpeed::Slow,
        };

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load(rock_sprite),
                ..default()
            },
            Rock {
                size: rock_size,
                speed: rock_speed,
            },
        ));
    }
}

pub fn tick_rock_spawn_timer(mut rock_spawn_timer: ResMut<RockSpawnTimer>, time: Res<Time>) {
    rock_spawn_timer.timer.tick(time.delta());
}

pub fn player_rock_collision() {}

pub fn move_rocks(mut rock_query: Query<(&mut Transform, &Rock)>, time: Res<Time>) {
    for (mut transform, rock) in rock_query.iter_mut() {
        let rock_speed = match rock.speed {
            RockSpeed::Fast => FAST_ROCK_SPEED,
            RockSpeed::Normal => NORMAL_ROCK_SPEED,
            RockSpeed::Slow => SLOW_ROCK_SPEED,
        };
        transform.translation.y -= rock_speed * time.delta_seconds();
    }
}

pub fn remove_rocks(
    mut commands: Commands,
    rock_query: Query<(Entity, &Transform, &Rock), With<Rock>>,
) {
    for (rock_entity, rock_transform, rock) in rock_query.iter() {
        let rock_size = match rock.size {
            RockSize::Big => BIG_ROCK_SIZE,
            RockSize::Normal => NORMAL_ROCK_SIZE,
            RockSize::Small => SMALL_ROCK_SIZE,
        };
        if rock_transform.translation.y < 0.0 - rock_size {
            commands.entity(rock_entity).despawn();
        }
    }
}
