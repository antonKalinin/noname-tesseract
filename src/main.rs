mod components;
mod constants;
mod systems;
mod utils;

use bevy::{prelude::*, render::pass::ClearColor};
use bevy_prototype_lyon::prelude::*;
// use rust_bert::pipelines;

use std::process;

use constants::SCALE_FACTOR;
use systems::{keyboard, mouse, setup};
use utils::get_clipboard_image;

fn main() {
    let image = match get_clipboard_image() {
        Some(image) => image,
        None => {
            println!("Could not get image from clipboard 😬");
            process::exit(1);
        }
    };
    let (image_width, image_height) = image.dimensions();

    // blur_image(image).unwrap();

    let screen_width = (image_width as f32) / SCALE_FACTOR;
    let screen_height = (image_height as f32) / SCALE_FACTOR;

    let window = WindowDescriptor {
        title: "🙈".to_string(),
        width: screen_width,
        height: screen_height,
        ..Default::default()
    };

    let mut app = App::build();

    app.insert_resource(window)
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(mouse.system())
        .add_system(keyboard.system())
        .run();
}
