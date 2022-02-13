mod components;
mod constants;
mod systems;
mod utils;

use bevy::prelude::*;
use bevy::window::WindowResizeConstraints;
use bevy_prototype_lyon::prelude::*;
use std::process;

use constants::SCALE_FACTOR;
use systems::{keyboard, mouse, setup, ui};

fn main() {
    let image = match utils::get_clipboard_image() {
        Some(image) => image,
        None => {
            println!("No image in clipboard. Nothing to do for me 🤓");
            process::exit(1);
        }
    };
    let (image_width, image_height) = image.dimensions();

    let screen_width = (image_width as f32) / SCALE_FACTOR;
    let screen_height = (image_height as f32) / SCALE_FACTOR;

    let window = WindowDescriptor {
        title: "🙈".to_string(),
        width: screen_width,
        height: screen_height,
        resize_constraints: WindowResizeConstraints {
            min_width: 1.,
            min_height: 1.,
            ..Default::default()
        },
        decorations: false,
        transparent: true,
        ..Default::default()
    };

    let log_settings = bevy::log::LogSettings {
        level: bevy::log::Level::ERROR,
        ..Default::default()
    };

    let mut app = App::new();

    app.insert_resource(window)
        .insert_resource(log_settings)
        .insert_resource(image)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(keyboard)
        .add_system(mouse)
        .add_system_to_stage(CoreStage::PostUpdate, ui)
        .run();
}
