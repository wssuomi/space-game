use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
    assets::{AudioAssets, SpriteAssets},
    explosion::SpawnExplosion,
    rock::{Rock, RocksDestroyed},
    score::Score,
    space_crates::{SpaceCrate, CRATE_DAMAGE, CRATE_HEAL, CRATE_HEIGHT, CRATE_WIDTH},
    state::AppState,
    ASSET_SCALE,
};

use bevy::{prelude::*, sprite::collide_aabb::collide};

pub const PLAYER_SPEED: f32 = 480.0;
pub const PLAYER_SIZE: f32 = 16.0 * ASSET_SCALE;
pub const PLAYER_STARTING_HEALTH: f32 = 100.0;

#[derive(Component)]
pub struct Player {
    pub health: f32,
}

pub struct HealPlayer {
    healing: f32,
}

pub struct DamagePlayer {
    damage: f32,
}

pub struct UpdatePlayerHealth;

pub fn spawn_player(mut commands: Commands, handles: Res<SpriteAssets>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0).with_scale(
                Vec3 {
                    x: ASSET_SCALE,
                    y: ASSET_SCALE,
                    ..default()
                },
            ),
            texture: handles.player.clone(),
            ..default()
        },
        Player {
            health: PLAYER_STARTING_HEALTH,
        },
    ));
    println!("player spawned");
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
    mut event_writer: EventWriter<DamagePlayer>,
    mut explosion_event_writer: EventWriter<SpawnExplosion>,
    mut score: ResMut<Score>,
    audio: Res<Audio>,
    handles: Res<AudioAssets>,
    mut rocks_destroyed: ResMut<RocksDestroyed>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (rock_entity, rock_transform, rock) in rock_query.iter() {
            let distance = player_transform
                .translation
                .distance(rock_transform.translation);
            if distance < PLAYER_SIZE / 2.0 + rock.size() / 2.0 {
                score.value += 25;
                println!("Score: {}", score.value);
                event_writer.send(DamagePlayer {
                    damage: rock.damage(),
                });
                explosion_event_writer.send(SpawnExplosion {
                    pos: Vec3::new(
                        rock_transform.translation.x,
                        rock_transform.translation.y,
                        2.0,
                    ),
                });
                audio.play(handles.rock_collison.clone());
                commands.entity(rock_entity).despawn();
                rocks_destroyed.count += 1;
            }
        }
    }
}

pub fn player_crate_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    crate_query: Query<(Entity, &Transform, &SpaceCrate), With<SpaceCrate>>,
    mut repair_event_writer: EventWriter<HealPlayer>,
    mut explosive_event_writer: EventWriter<DamagePlayer>,
    audio: Res<Audio>,
    handles: Res<AudioAssets>,
    mut score: ResMut<Score>,
    mut explosion_event_writer: EventWriter<SpawnExplosion>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, space_crate_transform, space_crate) in crate_query.iter() {
            if collide(
                player_transform.translation,
                Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
                space_crate_transform.translation,
                Vec2::new(CRATE_WIDTH, CRATE_HEIGHT),
            )
            .is_some()
            {
                match space_crate.crate_type {
                    crate::space_crates::CrateType::Health => {
                        repair_event_writer.send(HealPlayer {
                            healing: CRATE_HEAL,
                        });
                        audio.play(handles.collect_repair.clone());
                        commands.entity(entity).despawn();
                    }
                    crate::space_crates::CrateType::Explosive => {
                        explosive_event_writer.send(DamagePlayer {
                            damage: CRATE_DAMAGE,
                        });
                        audio.play(handles.hit_explosive.clone());
                        commands.entity(entity).despawn();
                        explosion_event_writer.send(SpawnExplosion {
                            pos: Vec3::new(
                                space_crate_transform.translation.x,
                                space_crate_transform.translation.y,
                                2.0,
                            ),
                        });
                    }
                }
                score.value += 100;
            }
        }
    }
}

pub fn heal_player(
    mut event_reader: EventReader<HealPlayer>,
    mut player_query: Query<&mut Player, With<Player>>,
    mut update_health_event_writer: EventWriter<UpdatePlayerHealth>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        for event in event_reader.iter() {
            player.health += event.healing;
            if player.health >= 100.0 {
                player.health = 100.0;
            }
            println!("Player health: {}", player.health);
            update_health_event_writer.send(UpdatePlayerHealth {});
        }
    }
}

pub fn damage_player(
    mut event_reader: EventReader<DamagePlayer>,
    mut player_query: Query<&mut Player, With<Player>>,
    mut update_health_event_writer: EventWriter<UpdatePlayerHealth>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        for event in event_reader.iter() {
            player.health -= event.damage;
            println!("damage: {}", event.damage);
            println!("player health: {}", player.health);
            update_health_event_writer.send(UpdatePlayerHealth {});
            if player.health <= 0.0 {
                next_app_state.set(AppState::MainMenu);
                println!("Ship Exploded.");
            }
        }
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_event::<DamagePlayer>()
            .add_event::<HealPlayer>()
            .add_event::<UpdatePlayerHealth>()
            .add_systems(
                (
                    player_crate_collision,
                    player_movement,
                    player_rock_collision,
                    damage_player,
                    heal_player,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
