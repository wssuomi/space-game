use bevy::prelude::*;

use crate::{assets::SpriteAssets, ASSET_SCALE};

pub const EXPLOSION_LENGTH: f32 = 0.2;

pub struct SpawnExplosion {
    pub pos: Vec3,
}

#[derive(Component)]
pub struct Explosion {
    pub timer: Timer,
}

impl Default for Explosion {
    fn default() -> Self {
        return Explosion {
            timer: Timer::from_seconds(EXPLOSION_LENGTH, TimerMode::Once),
        };
    }
}

fn spawn_explosion_on_event(
    mut commands: Commands,
    mut explosion_event_reader: EventReader<SpawnExplosion>,
    handles: Res<SpriteAssets>,
) {
    for e in explosion_event_reader.iter() {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: e.pos,
                    scale: Vec3::new(ASSET_SCALE, ASSET_SCALE, 1.0),
                    ..default()
                },
                texture: handles.explosion.clone(),
                ..default()
            },
            Explosion::default(),
        ));
    }
}

fn tick_explosion_timers(
    mut commands: Commands,
    mut explosion_query: Query<(Entity, &mut Explosion), With<Explosion>>,
    time: Res<Time>,
) {
    for (entity, mut e) in explosion_query.iter_mut() {
        e.timer.tick(time.delta());
        if e.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplosion>()
            .add_systems((spawn_explosion_on_event, tick_explosion_timers));
    }
}
