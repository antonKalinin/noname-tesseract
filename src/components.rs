use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct RectSize {
    pub width: f32,
    pub height: f32,
}

impl RectSize {
    pub fn new(width: f32, height: f32) -> RectSize {
        RectSize { width, height }
    }
}

#[derive(Component, Debug)]
pub struct TextRect {
    pub hovered: bool,
    pub selected: bool,
}

impl Default for TextRect {
    fn default() -> TextRect {
        TextRect {
            hovered: false,
            selected: false,
        }
    }
}
