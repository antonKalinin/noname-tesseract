use crate::components::{RectSize, TextRect};
use crate::constants::SCALE_FACTOR;
use crate::utils;
use arboard::{Clipboard, ImageData};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use image::{imageops, ImageBuffer, Rgba, RgbaImage};
use std::process;
use std::{borrow::Cow, io::Read};

pub fn keyboard(
    mut windows: ResMut<Windows>,
    mut image_res: ResMut<RgbaImage>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&TextRect, &Transform, &RectSize)>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        process::exit(1);
    }

    if keyboard_input.pressed(KeyCode::Return) {
        let window = windows.get_primary_mut().unwrap();
        let image = &mut *image_res;
        let (image_width, image_height) = image.dimensions();

        for (text_rect, transform, size) in query.iter() {
            if !text_rect.selected {
                continue;
            }

            let (rect_x, rect_y, _) = <(f32, f32, f32)>::from(transform.translation);
            let (x, y) = <(f32, f32)>::from(utils::rect_xy_to_screenshot_xy(
                Vec2::new(rect_x, rect_y),
                window,
            ));
            let width = (size.width * SCALE_FACTOR) as u32;
            let height = (size.height * SCALE_FACTOR) as u32;
            let overlay = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 255]));

            // For the future: blur instead of patch
            // let sub_image = SubImage::new(image, x as u32, y as u32, width, height);
            // let blurred_sub_image = imageops::blur(sub_image, 10.0);

            imageops::replace(image, &overlay, x as u32, y as u32);
        }

        let mut clipboard = Clipboard::new().unwrap();
        let image_bytes = image.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
        let clipboard_image = ImageData {
            width: image_width as usize,
            height: image_height as usize,
            bytes: Cow::from(image_bytes),
        };
        match clipboard.set_image(clipboard_image) {
            Ok(()) => println!("Image is copied back to clipboard 👌"),
            _ => println!("Could not copy image to clipboard 🙁"),
        };

        process::exit(1);
    }
}
