use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};
use bevy_flycam::FlyCam;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use commander::{camera::setup_commander_camera, control::move_camera};
use loading::loading::{check_assets_ready, setup_loading};
use sprites::sprite::{animate_sprite, rotate_sprites_to_camera, spawn_units};
use state::GameState;

pub mod commander;
pub mod loading;
pub mod sprites;
pub mod state;
pub mod world;

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
        .add_plugins(bevy_flycam::PlayerPlugin)
        .add_plugins(Sprite3dPlugin)
        .init_state::<GameState>()
        .add_plugins(WorldInspectorPlugin::new())
        // .add_systems(Startup, setup_commander_camera)
        .add_systems(Update, rotate_sprites_to_camera)
        .add_systems(Update, (move_camera, animate_sprite))
        .add_systems(PreStartup, setup_loading)
        .add_systems(OnEnter(GameState::Game), spawn_units)
        .add_systems(
            Update,
            check_assets_ready.run_if(in_state(GameState::Loading)),
        )
        .run();
}
