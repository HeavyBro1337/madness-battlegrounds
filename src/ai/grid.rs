use std::{
    marker::PhantomData,
};

use bevy::{prelude::*};

use super::grid_loc::GridLocation;

const GRID_SIZE: usize = 20;

#[derive(Resource)]
pub struct Grid<T> {
    pub entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
    _marker: PhantomData<T>,
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Entity, GridLocation)> + '_ {
        self.entities
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, entity)| entity.is_some())
            .map(|(i, entity)| {
                (
                    entity.unwrap(),
                    GridLocation(IVec2::new((i / GRID_SIZE) as i32, (i % GRID_SIZE) as i32)),
                )
            })
    }
}

#[derive(Event)]
pub struct DirtyGridEvent<T>(pub GridLocation, PhantomData<T>);

pub fn remove_from_grid<T: Component>(
    grid: ResMut<Grid<T>>,
    mut q_rm_components: RemovedComponents<T>,
    dirty: EventWriter<DirtyGridEvent<T>>,
) {
    for removed_entity in q_rm_components.read() {}
}
