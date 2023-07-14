use bevy::prelude::*;

use crate::{arena::ARENA_HEIGHT, assets::SpriteAssets, player::Player};

pub const BULLET_SPEED: f32 = 100.0;
pub const BULLET_HEIGHT: f32 = 30.0;

#[derive(Component)]
pub struct Bullet;

pub fn shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    handles: Res<SpriteAssets>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(transform) = player_query.get_single() {
        if keyboard_input.pressed(KeyCode::Space) {
            commands.spawn((
                SpriteBundle {
                    transform: *transform,
                    texture: handles.bullet.clone(),
                    ..default()
                },
                Bullet {},
            ));
        }
    }
}

pub fn move_bullets(mut bullet_query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut transform in bullet_query.iter_mut() {
        transform.translation.y += BULLET_SPEED * time.delta_seconds();
    }
}

pub fn despawn_off_screen_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        if bullet_transform.translation.y > ARENA_HEIGHT + BULLET_HEIGHT {
            commands.entity(bullet_entity).despawn();
            println!("bullet despawned");
        }
    }
}

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((shoot, move_bullets, despawn_off_screen_bullets));
    }
}
