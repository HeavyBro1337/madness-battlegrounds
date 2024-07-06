use bevy::prelude::*;

#[derive(Resource, States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Loading = 0,
    Game = 1,
}
