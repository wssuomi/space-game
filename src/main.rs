use bevy::prelude::*;

pub mod player;
pub mod rock;
pub mod star;

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
        .init_resource::<rock::RockSpawnTimer>()
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
        .add_startup_system(star::spawn_stars)
        .add_startup_system(spawn_arena_walls)
        .add_system(rock::spawn_rocks_over_time)
        .add_system(rock::tick_rock_spawn_timer)
        .add_system(player::player_rock_collision)
        .add_system(player::player_movement)
        .add_system(rock::move_rocks)
        .add_system(rock::remove_rocks)
        .add_system(tick_score_timer)
        .add_system(add_score_over_timer)
        .add_system(star::move_stars)
        .add_system(star::send_star_to_top)
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

pub fn add_score_over_timer(mut score: ResMut<Score>, score_timer: Res<ScoreTimer>) {
    if score_timer.timer.finished() {
        score.value += 5;
        println!("Score: {}", score.value);
    }
}

pub fn tick_score_timer(mut score_timer: ResMut<ScoreTimer>, time: Res<Time>) {
    score_timer.timer.tick(time.delta());
}
