use crate::components::TextRect;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::DrawMode;

pub fn state(mut query: Query<(&TextRect, &mut DrawMode)>) {
    for (text_rect, mut draw_mode) in query.iter_mut() {
        if let DrawMode::Fill(ref mut fill_mode) = *draw_mode {
            match text_rect {
                TextRect {
                    hovered: true,
                    selected: false,
                } => {
                    fill_mode.color = Color::rgba(1.0, 0.07, 0.57, 0.5);
                }
                TextRect { selected: true, .. } => {
                    fill_mode.color = Color::BLACK;
                }
                _ => {
                    fill_mode.color = Color::NONE;
                }
            }
        }
    }
}
