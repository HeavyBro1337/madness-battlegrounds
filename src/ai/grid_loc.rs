use std::{
    ops::{Index, IndexMut},
};

use bevy::prelude::*;

use super::grid::Grid;

#[derive(Debug, Component, Clone, Default, Deref, DerefMut, Hash, PartialEq, Eq)]
pub struct GridLocation(pub IVec2);

impl<T> Index<&GridLocation> for Grid<T> {
    type Output = Option<Entity>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.entities[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<&GridLocation> for Grid<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.entities[index.x as usize][index.y as usize]
    }
}
