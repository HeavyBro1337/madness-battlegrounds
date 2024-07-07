use std::sync::{Arc, RwLock};

use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use futures_lite::future;
use oxidized_navigation::{debug_draw::DrawPath, query::{self, find_path}, tiles::NavMeshTiles, NavMesh, NavMeshSettings};

use crate::commander::cursor::Cursor3DCoordinates;

#[derive(Default, Resource)]
pub struct AsyncPathfindingTasks {
    tasks: Vec<Task<Option<Vec<Vec3>>>>,
}

#[derive(Component)]
pub struct AiUnit;

pub fn generate_path_to_cursor(
    cursor: Res<Cursor3DCoordinates>,
    q_units: Query<(&Transform), With<AiUnit>>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut pathfinding_task: ResMut<AsyncPathfindingTasks>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    let nav_mesh_lock = nav_mesh.get();

    let end_pos = cursor.0;

    let thread_pool = AsyncComputeTaskPool::get();

    for transform in q_units.iter() {
        let task = thread_pool.spawn(async_path_find(
            nav_mesh_lock.clone(),
            nav_mesh_settings.clone(),
            transform.translation,
            end_pos,
            None,
        ));

        pathfinding_task.tasks.push(task);
    }
}

async fn async_path_find(
    nav_mesh_lock: Arc<RwLock<NavMeshTiles>>,
    nav_mesh_settings: NavMeshSettings,
    start_pos: Vec3,
    end_pos: Vec3,
    position_search_radius: Option<f32>,
) -> Option<Vec<Vec3>> {
    // Get the underlying nav_mesh.
    let Ok(nav_mesh) = nav_mesh_lock.read() else {
        return None;
    };

    // Run pathfinding to get a path.
    match find_path(
        &nav_mesh,
        &nav_mesh_settings,
        start_pos,
        end_pos,
        position_search_radius,
        Some(&[1.0, 0.5]),
    ) {
        Ok(path) => {
            info!("Found path (ASYNC): {:?}", path);
            return Some(path);
        }
        Err(error) => error!("Error with pathfinding: {:?}", error),
    }

    None
}

pub fn poll_pathfinding_tasks_system(
    mut commands: Commands,
    mut pathfinding_task: ResMut<AsyncPathfindingTasks>,
) {
    // Go through and remove completed tasks.
    pathfinding_task.tasks.retain_mut(|task| {
        if let Some(string_path) = future::block_on(future::poll_once(task)).unwrap_or(None) {
            info!("Async path task finished with result: {:?}", string_path);
            commands.spawn(DrawPath {
                timer: Some(Timer::from_seconds(4.0, TimerMode::Once)),
                pulled_path: string_path,
                color: Color::BLUE,
            });

            false
        } else {
            true
        }
    });
}
