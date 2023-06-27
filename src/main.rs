use bevy::prelude::*;
use rand::prelude::*;

pub mod player;

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
pub const CLEAR_COLOR: Color = Color::rgb(0.0, 0.0, 0.2);

pub const TOP_WALL_X: f32 = 450.0;
pub const BOTTOM_WALL_X: f32 = 450.0;
pub const LEFT_WALL_X: f32 = 0.0;
pub const RIGHT_WALL_X: f32 = 900.0;

pub const TOP_WALL_Y: f32 = 900.0;
pub const BOTTOM_WALL_Y: f32 = 0.0;
pub const LEFT_WALL_Y: f32 = 450.0;
pub const RIGHT_WALL_Y: f32 = 450.0;

pub const HIDE_ARENA_OVERFLOW_AREA_THICKNESS: f32 = 200.0;
pub const WALL_THICKNESS: f32 = 10.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .init_resource::<RockSpawnTimer>()
        .init_resource::<Score>()
        .init_resource::<ScoreTimer>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space game".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(player::spawn_player)
        .add_startup_system(spawn_stars)
        .add_startup_system(spawn_arena_walls)
        .add_system(spawn_rocks_over_time)
        .add_system(tick_rock_spawn_timer)
        .add_system(player::player_rock_collision)
        .add_system(player::player_movement)
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

#[derive(Component)]
pub struct VoidBox {}

#[derive(Bundle)]
pub struct VoidBoxBundle {
    sprite_bundle: SpriteBundle,
}

enum ArenaWallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl ArenaWallLocation {
    fn position(&self, wall_thickness: f32) -> Vec3 {
        match self {
            ArenaWallLocation::Left => Vec3::new(LEFT_WALL_X - wall_thickness / 2.0, 450.0, 3.0),
            ArenaWallLocation::Right => Vec3::new(RIGHT_WALL_X + wall_thickness / 2.0, 450.0, 3.0),
            ArenaWallLocation::Top => Vec3::new(450.0, TOP_WALL_Y + wall_thickness / 2.0, 3.0),
            ArenaWallLocation::Bottom => {
                Vec3::new(450.0, BOTTOM_WALL_Y - wall_thickness / 2.0, 3.0)
            }
        }
    }

    fn size(&self, wall_thickness: f32) -> Vec3 {
        match self {
            ArenaWallLocation::Top | ArenaWallLocation::Bottom => {
                Vec3::new(ARENA_WIDTH, wall_thickness, 1.0)
            }
            ArenaWallLocation::Left | ArenaWallLocation::Right => {
                Vec3::new(wall_thickness, ARENA_HEIGHT + wall_thickness * 2.0, 1.0)
            }
        }
    }
}

impl ArenaWallBundle {
    fn new(location: ArenaWallLocation, wall_thickness: f32, wall_color: Color) -> ArenaWallBundle {
        ArenaWallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position(wall_thickness),
                    scale: location.size(wall_thickness),
                    ..default()
                },
                sprite: Sprite {
                    color: wall_color,
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 5.0),
        ..default()
    });
}

pub fn spawn_arena_walls(mut commands: Commands) {
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Bottom,
            HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
            CLEAR_COLOR,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Top,
            HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
            CLEAR_COLOR,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Left,
            HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
            CLEAR_COLOR,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Right,
            HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
            CLEAR_COLOR,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(ArenaWallLocation::Top, WALL_THICKNESS, WALL_COLOR),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(ArenaWallLocation::Bottom, WALL_THICKNESS, WALL_COLOR),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(ArenaWallLocation::Right, WALL_THICKNESS, WALL_COLOR),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(ArenaWallLocation::Left, WALL_THICKNESS, WALL_COLOR),
        ArenaWall {},
    ));
}

pub fn spawn_rocks_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rock_spawn_timer: Res<RockSpawnTimer>,
) {
    if rock_spawn_timer.timer.finished() {
        let random_x = random::<f32>() * ARENA_WIDTH;

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

pub fn spawn_stars(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn send_star_to_top(mut star_query: Query<&mut Transform, With<Star>>) {
    for mut transform in star_query.iter_mut() {
        if transform.translation.y < -10.0 {
            transform.translation.y = ARENA_HEIGHT + 10.0;
        }
    }
}
