use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct SpriteAssets {
    pub player: Handle<Image>,
    pub big_rock: Handle<Image>,
    pub normal_rock: Handle<Image>,
    pub small_rock: Handle<Image>,
    pub star: Handle<Image>,
    pub health_crate: Handle<Image>,
    pub explosive_crate: Handle<Image>,
}

#[derive(Resource)]
pub struct AudioAssets {
    pub player_rock_collison: Handle<AudioSource>,
    pub collect_repair: Handle<AudioSource>,
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
        health_crate: asset_server.load("sprites/health_box.png"),
        explosive_crate: asset_server.load("sprites/explosive_box.png"),
    };
    commands.insert_resource(sprite_assets);
    let audio_assets = AudioAssets {
        player_rock_collison: asset_server.load("audio/rock_hit.ogg"),
        collect_repair: asset_server.load("audio/collect_repair.ogg"),
    };
    commands.insert_resource(audio_assets);
}
