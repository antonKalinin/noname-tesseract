use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use leptess::{leptonica, tesseract};
use std::path::Path;

use crate::components::TextRect;
use crate::constants::{IMAGE_DPI, SCALE_FACTOR};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

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

    let boxes = tess_api
        .get_component_images(leptess::capi::TessPageIteratorLevel_RIL_WORD, true)
        .unwrap();

    for bbox in &boxes {
        tess_api.set_rectangle(&bbox);
        let text = tess_api.get_utf8_text().unwrap();
        let confidence = tess_api.mean_text_conf();

        // this is most likely a text line
        let bounding_box = bbox.as_ref();
        let x = (bounding_box.x as f32) / SCALE_FACTOR - (window.width() / 2.0);
        let y = (window.height() / 2.0) - (bounding_box.y as f32) / SCALE_FACTOR;
        let width = (bounding_box.w as f32) / SCALE_FACTOR;
        let height = (bounding_box.h as f32) / SCALE_FACTOR;

        // let shape = shapes::Rectangle {
        //     width,
        //     height,
        //     origin: shapes::RectangleOrigin::TopLeft,
        // };

        // commands
        //     .spawn_bundle(GeometryBuilder::build_as(
        //         &shape,
        //         ShapeColors::outlined(Color::NONE, Color::rgb(1.0, 0.07, 0.57)),
        //         DrawMode::Outlined {
        //             fill_options: FillOptions::default(),
        //             outline_options: StrokeOptions::default().with_line_width(1.0),
        //         },
        //         Transform::from_xyz(x, y, 0.0),
        //     ))
        //     .insert(TextRect::default());
    }
}
