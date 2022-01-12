use image::{imageops, ImageError, Rgb, RgbImage, SubImage};
use leptess::{leptonica, tesseract};
use std::{path::Path, process};

const TEXT_CONFIDENCE_LEVEL: i32 = 90;
const IMAGE_DPI: i32 = 72;

fn blur_image(mut image: RgbImage) -> Result<(), ImageError> {
    let mut tess_api = tesseract::TessApi::new(None, "eng").unwrap();
    let pix = leptonica::pix_read(Path::new("assets/tmp.png")).unwrap();

    tess_api.set_image(&pix);
    tess_api.set_source_resolution(IMAGE_DPI);

    let boxes = tess_api
        .get_component_images(leptess::capi::TessPageIteratorLevel_RIL_WORD, true)
        .unwrap();

    for bbox in &boxes {
        tess_api.set_rectangle(&bbox);
        let confidence = tess_api.mean_text_conf();

        if confidence > TEXT_CONFIDENCE_LEVEL {
            // this is most likely a text line
            let bounding_box = bbox.as_ref();
            let x = bounding_box.x as u32;
            let y = bounding_box.y as u32;
            let width = bounding_box.w as u32;
            let height = bounding_box.h as u32;
            let sub_image = SubImage::new(&image, x, y, width, height);
            let blurred_sub_image = imageops::blur(&sub_image, 10.0);

            imageops::replace(&mut image, &blurred_sub_image, x, y)
        }
    }

    image.save("assets/tmp_blurred.png").unwrap();

    Ok(())
}
