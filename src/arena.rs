use bevy::prelude::*;

use crate::CLEAR_COLOR;

pub const ARENA_WIDTH: f32 = 900.0;
pub const ARENA_HEIGHT: f32 = 900.0;

pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub const LEFT_WALL_X: f32 = 0.0;
pub const RIGHT_WALL_X: f32 = 900.0;

pub const TOP_WALL_Y: f32 = 900.0;
pub const BOTTOM_WALL_Y: f32 = 0.0;

pub const HIDE_ARENA_OVERFLOW_AREA_THICKNESS: f32 = 200.0;
pub const WALL_THICKNESS: f32 = 10.0;

pub const WALL_LAYER: f32 = 3.0;
pub const HIDE_OVERFLOW_LAYER: f32 = 2.0;

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
    fn position(&self, wall_thickness: f32, draw_layer: f32) -> Vec3 {
        match self {
            ArenaWallLocation::Left => {
                Vec3::new(LEFT_WALL_X - wall_thickness / 2.0, 450.0, draw_layer)
            }
            ArenaWallLocation::Right => {
                Vec3::new(RIGHT_WALL_X + wall_thickness / 2.0, 450.0, draw_layer)
            }
            ArenaWallLocation::Top => {
                Vec3::new(450.0, TOP_WALL_Y + wall_thickness / 2.0, draw_layer)
            }
            ArenaWallLocation::Bottom => {
                Vec3::new(450.0, BOTTOM_WALL_Y - wall_thickness / 2.0, draw_layer)
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
    fn new(
        location: ArenaWallLocation,
        wall_thickness: f32,
        wall_color: Color,
        draw_layer: f32,
    ) -> ArenaWallBundle {
        ArenaWallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position(wall_thickness, draw_layer),
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

pub fn spawn_arena_walls(mut commands: Commands) {
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Bottom,
            HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
            CLEAR_COLOR,
            HIDE_OVERFLOW_LAYER,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Top,
            HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
            CLEAR_COLOR,
            HIDE_OVERFLOW_LAYER,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Left,
            HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
            CLEAR_COLOR,
            HIDE_OVERFLOW_LAYER,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Right,
            HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
            CLEAR_COLOR,
            HIDE_OVERFLOW_LAYER,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Top,
            WALL_THICKNESS,
            WALL_COLOR,
            WALL_LAYER,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Bottom,
            WALL_THICKNESS,
            WALL_COLOR,
            WALL_LAYER,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Right,
            WALL_THICKNESS,
            WALL_COLOR,
            WALL_LAYER,
        ),
        ArenaWall {},
    ));
    commands.spawn((
        ArenaWallBundle::new(
            ArenaWallLocation::Left,
            WALL_THICKNESS,
            WALL_COLOR,
            WALL_LAYER,
        ),
        ArenaWall {},
    ));
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_arena_walls);
    }
}
