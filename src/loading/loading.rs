use bevy::{asset, prelude::*, utils::HashMap};

use crate::state::GameState;

#[derive(Resource, Default)]
pub struct LoadingAssets(Vec<UntypedHandle>);

#[derive(Resource, Default)]
pub struct ImageLayouts(pub HashMap<String, Handle<TextureAtlasLayout>>);

pub fn setup_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut loading_assets = LoadingAssets::default();
    let mut image_layouts = ImageLayouts::default();
    loading_assets.0.push(
        asset_server
            .load::<Image>("sprites/unit_idle.png")
            .untyped(),
    );
    loading_assets.0.push(
        asset_server
            .load::<Image>("sprites/unit_walk.png")
            .untyped(),
    );
    commands.insert_resource(loading_assets);
    image_layouts.0.insert(
        "sprites/unit_idle.png".to_string(),
        atlases.add(TextureAtlasLayout::from_grid(
            Vec2::ONE * 100.0,
            6,
            1,
            None,
            None,
        )),
    );
    commands.insert_resource(image_layouts);
}

pub fn check_assets_ready(
    server: Res<AssetServer>,
    loading: Res<LoadingAssets>,
    mut state: ResMut<NextState<GameState>>,
) {
    use bevy::asset::LoadState;

    if loading
        .0
        .iter()
        .all(|asset| match server.get_load_state(asset.id()).unwrap() {
            LoadState::Loaded => true,
            _ => false,
        })
    {
        println!("loaded");
        state.set(GameState::Game);
    }
}
