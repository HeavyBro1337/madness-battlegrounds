use ai::{
    ai::{generate_path_to_cursor, poll_pathfinding_tasks_system, AsyncPathfindingTasks},
    r#move::move_agents,
};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::Collider,
    render::RapierDebugRenderPlugin,
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use commander::{
    camera::setup_commander_camera,
    control::move_camera,
    cursor::Cursor3dPlugin,
    selection::{init_selection, render_selection_box, update_selection},
};
use loading::loading::{check_assets_ready, setup_loading};
use oxidized_navigation::{
    debug_draw::OxidizedNavigationDebugDrawPlugin, NavMeshSettings, OxidizedNavigationPlugin,
};
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
        .add_plugins(Cursor3dPlugin)
        .init_state::<GameState>()
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings::from_agent_and_bounds(
                0.5, 1.9, 250.0, -10.0,
            )),
            OxidizedNavigationDebugDrawPlugin,
        ))
        .insert_resource(AsyncPathfindingTasks::default())
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_systems(Startup, (setup_commander_camera, init_selection))
        .add_systems(Update, rotate_sprites_to_camera)
        .add_systems(
            Update,
            (
                move_camera,
                animate_sprite,
                update_selection,
                render_selection_box,
                poll_pathfinding_tasks_system,
                generate_path_to_cursor,
                move_agents,
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
