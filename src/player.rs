use crate::{
    Rock, RockSize, Score, ARENA_HEIGHT, ARENA_WIDTH, BIG_ROCK_SIZE, NORMAL_ROCK_SIZE, PLAYER_SIZE,
    PLAYER_SPEED, SMALL_ROCK_SIZE,
};

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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