use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 480.0;
pub const PLAYER_SIZE: f32 = 100.0;
pub const ROCK_COOLDOWN: f32 = 2.0;
pub const SCORE_COOLDOWN: f32 = 1.0;
pub const FAST_ROCK_SPEED: f32 = 100.0;
pub const NORMAL_ROCK_SPEED: f32 = 75.0;
pub const SLOW_ROCK_SPEED: f32 = 50.0;
pub const BIG_ROCK_SIZE: f32 = 150.0;
pub const NORMAL_ROCK_SIZE: f32 = 100.0;
pub const SMALL_ROCK_SIZE: f32 = 70.0;
pub const STAR_COUNT: u32 = 100;
pub const STAR_SPEED: f32 = 40.0;
pub const ARENA_WIDTH: f32 = 900.0;
pub const ARENA_HEIGHT: f32 = 900.0;
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const WALL_SIZE: f32 = 10.0;

pub const TOP_WALL_X: f32 = 450.0;
pub const BOTTOM_WALL_X: f32 = 450.0;
pub const LEFT_WALL_X: f32 = 0.0;
pub const RIGHT_WALL_X: f32 = 900.0;

pub const TOP_WALL_Y: f32 = 900.0;
pub const BOTTOM_WALL_Y: f32 = 0.0;
pub const LEFT_WALL_Y: f32 = 450.0;
pub const RIGHT_WALL_Y: f32 = 450.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.2)))
        .init_resource::<RockSpawnTimer>()
        .init_resource::<Score>()
        .init_resource::<ScoreTimer>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space game".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                resizable: false,

                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_stars)
        .add_startup_system(spawn_arena_walls)
        .add_system(spawn_rocks_over_time)
        .add_system(tick_rock_spawn_timer)
        .add_system(player_rock_collision)
        .add_system(player_movement)
        .add_system(move_rocks)
        .add_system(remove_rocks)
        .add_system(tick_score_timer)
        .add_system(add_score_over_timer)
        .add_system(move_stars)
        .add_system(send_star_to_top)
        .run();
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct ScoreTimer {
    pub timer: Timer,
}

impl Default for ScoreTimer {
    fn default() -> Self {
        ScoreTimer {
            timer: Timer::from_seconds(SCORE_COOLDOWN, TimerMode::Repeating),
        }
    }
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
#[derive(Component)]
pub struct Star {}

#[derive(Component)]
pub struct ArenaWall {}

#[derive(Bundle)]
pub struct ArenaWallBundle {
    sprite_bundle: SpriteBundle,
}

enum ArenaWallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl ArenaWallLocation {
    fn position(&self) -> Vec3 {
        match self {
            ArenaWallLocation::Left => Vec3::new(LEFT_WALL_X - 5.0, LEFT_WALL_Y, 1.0),
            ArenaWallLocation::Right => Vec3::new(RIGHT_WALL_X + 5.0, RIGHT_WALL_Y, 1.0),
            ArenaWallLocation::Top => Vec3::new(TOP_WALL_X, TOP_WALL_Y + 5.0, 1.0),
            ArenaWallLocation::Bottom => Vec3::new(BOTTOM_WALL_X, BOTTOM_WALL_Y - 5.0, 1.0),
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            ArenaWallLocation::Top | ArenaWallLocation::Bottom => Vec3::new(ARENA_WIDTH, 10.0, 1.0),
            ArenaWallLocation::Left | ArenaWallLocation::Right => {
                Vec3::new(10.0, ARENA_HEIGHT + 20.0, 1.0)
            }
        }
    }
}

impl ArenaWallBundle {
    fn new(location: ArenaWallLocation) -> ArenaWallBundle {
        ArenaWallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position(),
                    scale: location.size(),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 5.0),
        ..default()
    });
}

pub fn spawn_arena_walls(mut commands: Commands) {
    commands.spawn((ArenaWallBundle::new(ArenaWallLocation::Top), ArenaWall {}));
    commands.spawn((
        ArenaWallBundle::new(ArenaWallLocation::Bottom),
        ArenaWall {},
    ));
    commands.spawn((ArenaWallBundle::new(ArenaWallLocation::Right), ArenaWall {}));
    commands.spawn((ArenaWallBundle::new(ArenaWallLocation::Left), ArenaWall {}));
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0),
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
        let x_max: f32 = ARENA_WIDTH - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = ARENA_HEIGHT - half_player_size;

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
        let random_x = random::<f32>() * ARENA_WIDTH;
        let random_y = random::<f32>() * ARENA_HEIGHT;

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
                transform: Transform::from_xyz(random_x, BIG_ROCK_SIZE + ARENA_HEIGHT, 0.0),
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

pub fn add_score_over_timer(mut score: ResMut<Score>, score_timer: Res<ScoreTimer>) {
    if score_timer.timer.finished() {
        score.value += 5;
        println!("Score: {}", score.value);
    }
}

pub fn tick_rock_spawn_timer(mut rock_spawn_timer: ResMut<RockSpawnTimer>, time: Res<Time>) {
    rock_spawn_timer.timer.tick(time.delta());
}

pub fn tick_score_timer(mut score_timer: ResMut<ScoreTimer>, time: Res<Time>) {
    score_timer.timer.tick(time.delta());
}

pub fn player_rock_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    rock_query: Query<(Entity, &Transform, &Rock), With<Rock>>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (rock_entity, rock_transform, rock) in rock_query.iter() {
            let rock_size = match rock.size {
                RockSize::Big => BIG_ROCK_SIZE,
                RockSize::Normal => NORMAL_ROCK_SIZE,
                RockSize::Small => SMALL_ROCK_SIZE,
            };
            let distance = player_transform
                .translation
                .distance(rock_transform.translation);
            if distance < PLAYER_SIZE / 2.0 + rock_size / 2.0 {
                score.value += 25;
                println!("Score: {}", score.value);
                let sound_effect = asset_server.load("audio/rock_hit.ogg");
                audio.play(sound_effect);
                commands.entity(rock_entity).despawn();
            }
        }
    }
}

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

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..STAR_COUNT {
        let random_x = random::<f32>() * ARENA_WIDTH;
        let random_y = random::<f32>() * ARENA_HEIGHT;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, -1.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn move_stars(mut star_query: Query<&mut Transform, With<Star>>, time: Res<Time>) {
    for mut transform in star_query.iter_mut() {
        transform.translation.y -= STAR_SPEED * time.delta_seconds();
    }
}

pub fn send_star_to_top(
    mut star_query: Query<&mut Transform, With<Star>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for mut transform in star_query.iter_mut() {
        if transform.translation.y < -10.0 {
            transform.translation.y = ARENA_HEIGHT + 10.0;
        }
    }
}
