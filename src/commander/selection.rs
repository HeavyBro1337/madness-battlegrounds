use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectionBox(Rect);

pub fn init_selection(mut commands: Commands) {
    commands.insert_resource(SelectionBox::default());
}

pub fn update_selection(
    gizmos: Gizmos,
    mouse: Res<ButtonInput<MouseButton>>,
    mut selection: ResMut<SelectionBox>,
) {
    if mouse.pressed(MouseButton::Left) {}
}
