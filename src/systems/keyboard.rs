use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use std::process;

pub fn keyboard(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Return) {
        println!("Image is copied back to clipboard 👌");
        process::exit(1);
    }
}
