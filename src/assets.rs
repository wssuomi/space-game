use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct SpriteAssets {
    pub player: Handle<Image>,
    pub big_rock: Handle<Image>,
    pub normal_rock: Handle<Image>,
    pub small_rock: Handle<Image>,
    pub star: Handle<Image>,
}

#[derive(Resource)]
pub struct AudioAssets {
    pub player_rock_collison: Handle<AudioSource>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_assets = SpriteAssets {
        player: asset_server.load("sprites/player.png"),
        big_rock: asset_server.load("sprites/big_rock.png"),
        normal_rock: asset_server.load("sprites/normal_rock.png"),
        small_rock: asset_server.load("sprites/small_rock.png"),
        star: asset_server.load("sprites/star.png"),
    };
    commands.insert_resource(sprite_assets);
    let audio_assets = AudioAssets {
        player_rock_collison: asset_server.load("audio/rock_hit.ogg"),
    };
    commands.insert_resource(audio_assets);
}