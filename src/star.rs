use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
    ASSET_SCALE,
};
use bevy::prelude::*;
use rand::prelude::*;

pub const STAR_COUNT: u32 = 100;
pub const STAR_SPEED: f32 = 40.0;

#[derive(Component)]
pub struct Star {}

pub fn spawn_stars(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..STAR_COUNT {
        let random_x = random::<f32>() * ARENA_WIDTH;
        let random_y = random::<f32>() * ARENA_HEIGHT;

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: random_x,
                        y: random_y,
                        z: -1.0,
                    },
                    scale: Vec3 {
                        x: ASSET_SCALE,
                        y: ASSET_SCALE,
                        ..default()
                    },
                    ..default()
                },
                // transform: Transform::from_xyz(random_x, random_y, -1.0),
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

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_stars)
            .add_systems((move_stars, send_star_to_top));
    }
}
