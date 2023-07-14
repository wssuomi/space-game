use std::time::Duration;

use bevy::prelude::*;

use crate::{arena::ARENA_HEIGHT, assets::SpriteAssets, player::Player, state::AppState};

pub const BULLET_SPEED: f32 = 100.0;
pub const BULLET_HEIGHT: f32 = 30.0;
pub const BULLET_COOLDOWN: f32 = 2.0;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletCooldownTimer {
    timer: Timer,
}

impl Default for BulletCooldownTimer {
    fn default() -> Self {
        let mut bullet_cooldown_timer = BulletCooldownTimer {
            timer: Timer::from_seconds(BULLET_COOLDOWN, TimerMode::Once),
        };

        bullet_cooldown_timer
            .timer
            .set_elapsed(Duration::from_secs_f32(BULLET_COOLDOWN));

        return bullet_cooldown_timer;
    }
}

pub fn shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    handles: Res<SpriteAssets>,
    player_query: Query<&Transform, With<Player>>,
    mut bullet_cooldown_timer: ResMut<BulletCooldownTimer>,
) {
    if let Ok(transform) = player_query.get_single() {
        if keyboard_input.pressed(KeyCode::Space) && bullet_cooldown_timer.timer.finished() {
            commands.spawn((
                SpriteBundle {
                    transform: *transform,
                    texture: handles.bullet.clone(),
                    ..default()
                },
                Bullet {},
            ));
            bullet_cooldown_timer.timer.reset();
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

pub fn despawn_all_bullets(mut commands: Commands, bullet_query: Query<Entity, With<Bullet>>) {
    for entity in bullet_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn add_bullet_cooldown_timer_resource(mut commands: Commands) {
    commands.insert_resource(BulletCooldownTimer::default())
}

pub fn remove_bullet_cooldown_timer_resource(mut commands: Commands) {
    commands.remove_resource::<BulletCooldownTimer>();
}

pub fn tick_bullet_cooldown_timer(
    mut bullet_cooldown_timer: ResMut<BulletCooldownTimer>,
    time: Res<Time>,
) {
    bullet_cooldown_timer.timer.tick(time.delta());
}

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_bullet_cooldown_timer_resource.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    shoot,
                    move_bullets,
                    despawn_off_screen_bullets,
                    tick_bullet_cooldown_timer,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_systems(
                (remove_bullet_cooldown_timer_resource, despawn_all_bullets)
                    .in_schedule(OnExit(AppState::Game)),
            );
    }
}
