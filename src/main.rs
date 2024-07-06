use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use commander::{
    camera::setup_commander_camera,
    control::move_camera,
    selection::{init_selection, render_selection_box, update_selection},
};
use loading::loading::{check_assets_ready, setup_loading};
use sprites::animation::animate_sprite;
use sprites::sprite::{rotate_sprites_to_camera, spawn_units};
use state::GameState;
use world::world::{spawn_map, spawn_sun};

mod ai;
mod commander;
mod loading;
mod sprites;
mod state;
mod world;

fn main() {
    App::new()
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Madness Battlegrounds".to_string(),
                        resolution: WindowResolution::new(1024.0, 600.0),
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugins(bevy_flycam::PlayerPlugin)
        .add_plugins(Sprite3dPlugin)
        .init_state::<GameState>()
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (setup_commander_camera, init_selection))
        .add_systems(Update, rotate_sprites_to_camera)
        .add_systems(
            Update,
            (
                move_camera,
                animate_sprite,
                update_selection,
                render_selection_box,
            ),
        )
        .add_systems(PreStartup, setup_loading)
        .add_systems(
            OnEnter(GameState::Game),
            (spawn_units, spawn_map, spawn_sun),
        )
        .add_systems(
            Update,
            check_assets_ready.run_if(in_state(GameState::Loading)),
        )
        .run();
}
