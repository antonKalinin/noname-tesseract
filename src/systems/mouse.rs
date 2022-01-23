use crate::components::{RectSize, TextRect};
use crate::utils;
use bevy::prelude::*;

pub fn mouse(
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<(&mut TextRect, &Transform, &RectSize)>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    // for event in mouse_button_input_events.iter() {}
    let cursor_event = cursor_moved_events.iter().next();

    for (mut text_rect, transform, size) in query.iter_mut() {
        let rect_xy = utils::rect_xy_to_screen_xy(
            Vec2::new(
                transform.translation.x,
                transform.translation.y - size.height,
            ),
            window,
        );

        if let Some(cursor) = cursor_event {
            let mouse_pos = cursor.position;
            let rect_wh = Vec2::new(size.width, size.height);

            if utils::is_point_in_rect(mouse_pos, rect_xy, rect_wh) {
                text_rect.hovered = true;

                if mouse_button_input.pressed(MouseButton::Left) {
                    text_rect.selected = !text_rect.selected;
                }
            } else {
                text_rect.hovered = false;
            }
        }
    }
}
