use arboard::Clipboard;
use image::{Rgb, RgbImage};

pub fn get_clipboard_image() -> Option<RgbImage> {
    let mut clipboard = Clipboard::new().unwrap();
    let image_data = match clipboard.get_image() {
        Ok(data) => data,
        Err(_) => return None,
    };

    let width = image_data.width as u32; // physical width
    let height = image_data.height as u32; // physical height

    let bytes = image_data.bytes;

    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let index = (y * (width * 4) + (x * 4)) as usize;
            if let [r, g, b] = &bytes[index..index + 3] {
                image.put_pixel(x, y, Rgb([*r, *g, *b]));
            };
        }
    }

    Some(image)
}
