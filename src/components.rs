use bevy::prelude::*;

#[derive(Component)]
pub struct TextRect {
    pub focused: bool,
    pub pressed: bool,
}

impl Default for TextRect {
    fn default() -> TextRect {
        TextRect {
            focused: false,
            pressed: false,
        }
    }
}
