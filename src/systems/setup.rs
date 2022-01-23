use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use leptess::{leptonica, tesseract};
use regex::Regex;
use std::path::Path;
use std::process;

use crate::components::{RectSize, TextRect};
use crate::constants::{IMAGE_DPI, SCALE_FACTOR};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let text_re = Regex::new(r"\w+").unwrap();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("tmp.png"),
        transform: Transform::from_scale(Vec3::splat(0.5)),
        ..Default::default()
    });

    let mut tess_api = tesseract::TessApi::new(None, "eng").unwrap();
    // TODO: Use pix_read_mem instead to read image directly from clipboard
    let pix = leptonica::pix_read(Path::new("assets/tmp.png")).unwrap();

    tess_api.set_image(&pix);
    tess_api.set_source_resolution(IMAGE_DPI);

    let boxes = tess_api.get_component_images(leptess::capi::TessPageIteratorLevel_RIL_WORD, true);

    if boxes.is_none() {
        println!("Couldn't find any text to hide 😬");
        process::exit(1);
    }

    for bbox in &boxes.unwrap() {
        tess_api.set_rectangle(&bbox);
        let text = tess_api.get_utf8_text().unwrap();
        let _confidence = tess_api.mean_text_conf();

        // if not a text line than skip
        if !text_re.is_match(&text) {
            continue;
        }

        let bounding_box = bbox.as_ref();
        let x = (bounding_box.x as f32) / SCALE_FACTOR - (window.width() / 2.0);
        let y = (window.height() / 2.0) - (bounding_box.y as f32) / SCALE_FACTOR;
        let width = (bounding_box.w as f32) / SCALE_FACTOR;
        let height = (bounding_box.h as f32) / SCALE_FACTOR;

        let rect = shapes::Rectangle {
            extents: Vec2::new(width, height),
            origin: shapes::RectangleOrigin::TopLeft,
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &rect,
                DrawMode::Fill(FillMode::color(Color::NONE)),
                Transform::from_xyz(x, y, 1.0),
            ))
            .insert(TextRect::default())
            .insert(RectSize::new(width, height));
    }
}
