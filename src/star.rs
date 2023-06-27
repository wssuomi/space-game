use crate::{ARENA_HEIGHT, ARENA_WIDTH, STAR_COUNT, STAR_SPEED};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct Star {}

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
