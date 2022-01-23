use crate::constants::SCALE_FACTOR;
use bevy::prelude::*;

pub fn is_point_in_rect(point: Vec2, rect_xy: Vec2, rect_wh: Vec2) -> bool {
    let (px, py) = <(f32, f32)>::from(point);
    let (rx, ry) = <(f32, f32)>::from(rect_xy);
    let (rw, rh) = <(f32, f32)>::from(rect_wh);

    px > rx && py > ry && px < (rx + rw) && py < (ry + rh)
}

pub fn rect_xy_to_screen_xy(rect_xy: Vec2, window: &Window) -> Vec2 {
    let x = rect_xy.x + (window.width() / 2.0);
    let y = (window.height() / 2.0) + rect_xy.y;

    Vec2::new(x, y)
}

pub fn rect_xy_to_screenshot_xy(rect_xy: Vec2, window: &Window) -> Vec2 {
    let x = (rect_xy.x + (window.width() / 2.0)) * SCALE_FACTOR;
    let y = (rect_xy.y + (window.height() / 2.0)) * SCALE_FACTOR;

    Vec2::new(x, y)
}
