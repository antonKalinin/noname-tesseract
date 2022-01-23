use arboard::Clipboard;
use image::{ImageBuffer, RgbaImage};

pub fn get_clipboard_image() -> Option<RgbaImage> {
    let mut clipboard = Clipboard::new().unwrap();
    let image_data = match clipboard.get_image() {
        Ok(data) => data,
        Err(_) => return None,
    };

    let image: RgbaImage = ImageBuffer::from_raw(
        image_data.width.try_into().unwrap(),
        image_data.height.try_into().unwrap(),
        image_data.bytes.into_owned(),
    )
    .unwrap();

    Some(image)
}
