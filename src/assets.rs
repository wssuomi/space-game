use bevy::{prelude::*, render::texture::ImageSampler};

#[derive(Debug, Resource)]
pub struct SpriteAssets {
    pub player: Handle<Image>,
    pub big_rock: Handle<Image>,
    pub normal_rock: Handle<Image>,
    pub small_rock: Handle<Image>,
    pub star: Handle<Image>,
    pub health_crate: Handle<Image>,
    pub explosive_crate: Handle<Image>,
    pub bullet: Handle<Image>,
    pub background: Handle<Image>,
    pub explosion: Handle<Image>,
}

#[derive(Resource)]
pub struct AudioAssets {
    pub rock_collison: Handle<AudioSource>,
    pub collect_repair: Handle<AudioSource>,
    pub hit_explosive: Handle<AudioSource>,
    pub shoot: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct UiAssets {
    pub menu_font: Handle<Font>,
}

fn fix_blurry_textures(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                let texture = assets.get_mut(&handle).unwrap();
                texture.sampler_descriptor = ImageSampler::nearest();
            }
            _ => {}
        }
    }
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(fix_blurry_textures);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_assets = SpriteAssets {
        player: asset_server.load("sprites/ship.png"),
        big_rock: asset_server.load("sprites/big_rock.png"),
        normal_rock: asset_server.load("sprites/normal_rock.png"),
        small_rock: asset_server.load("sprites/small_rock.png"),
        star: asset_server.load("sprites/star.png"),
        health_crate: asset_server.load("sprites/repair_crate.png"),
        explosive_crate: asset_server.load("sprites/explosive_crate.png"),
        bullet: asset_server.load("sprites/bullet.png"),
        background: asset_server.load("sprites/background.png"),
        explosion: asset_server.load("sprites/explosion.png"),
    };
    commands.insert_resource(sprite_assets);
    let audio_assets = AudioAssets {
        rock_collison: asset_server.load("audio/rock_hit.ogg"),
        collect_repair: asset_server.load("audio/collect_repair.ogg"),
        hit_explosive: asset_server.load("audio/hit_explosive.ogg"),
        shoot: asset_server.load("audio/shoot.ogg"),
    };
    commands.insert_resource(audio_assets);
    let ui_assets = UiAssets {
        menu_font: asset_server.load("fonts/m5x7.ttf"),
    };
    commands.insert_resource(ui_assets);
}
