use bevy::prelude::*;

use crate::{assets::SpriteAssets, player::Player};

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

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot);
    }
}
