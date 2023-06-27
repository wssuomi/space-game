use crate::{
    ARENA_HEIGHT, ARENA_WIDTH, BOTTOM_WALL_Y, CLEAR_COLOR, HIDE_ARENA_OVERFLOW_AREA_THICKNESS,
    LEFT_WALL_X, RIGHT_WALL_X, TOP_WALL_Y, WALL_COLOR, WALL_THICKNESS,
};
use bevy::prelude::*;

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
