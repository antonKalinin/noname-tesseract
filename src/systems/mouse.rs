use bevy::{input::mouse::MouseButtonInput, prelude::*, window::CursorMoved};

/// This system prints out all mouse events as they come in
pub fn mouse(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }
}
