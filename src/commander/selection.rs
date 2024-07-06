use bevy::prelude::*;

const SELECT_COLOR: Color = Color::LinearRgba(LinearRgba {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
    alpha: 0.5,
});

#[derive(Resource, Default)]
pub struct SelectionBox(Rect);

#[derive(Component)]
pub struct SelectionComponent;

pub fn init_selection(mut commands: Commands) {
    commands.insert_resource(SelectionBox::default());
    commands.spawn((
        NodeBundle {
            border_color: BorderColor(SELECT_COLOR),
            background_color: BackgroundColor(SELECT_COLOR),
            ..default()
        },
        SelectionComponent,
    ));
}

pub fn update_selection(
    q_window: Query<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut selection: ResMut<SelectionBox>,
) {
    let window = q_window.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) {
        selection.0.min = cursor_position;
    }

    if mouse.pressed(MouseButton::Left) {
        selection.0.max = cursor_position;
    }
}

pub fn render_selection_box(
    selection: Res<SelectionBox>,
    mut q_select: Query<&mut Style, With<SelectionComponent>>,
) {
    let mut select_style = q_select.single_mut();
    let min = selection.0.min;
    let max = selection.0.max;
    let lower_left = Vec2::new(min.x.min(max.x), min.y.min(max.y));
    let upper_right = Vec2::new(min.x.max(max.x), min.y.max(max.y));
    select_style.left = Val::Px(lower_left.x);
    select_style.top = Val::Px(lower_left.y);
    select_style.width = Val::Px((upper_right - lower_left).x);
    select_style.height = Val::Px((upper_right - lower_left).y);
}