use std::{slice, thread, time::Duration};
use image::{Bgra as Pixel, DynamicImage::ImageBgra8 as DynImage, ImageBuffer};
use minifb::{Window, WindowOptions};

type Image = ImageBuffer::<Pixel<u8>, Vec<u8>>;

fn render(image: &mut Image) {
    for (i, pixel) in image.pixels_mut().enumerate() {
        let color = i as u32;
        *pixel = Pixel(color.to_le_bytes());
    }
}

fn main() {
    let mut image = Image::new(640, 480);

    let mut window = Window::new(
        "crab rayve",
        image.width() as usize,
        image.height() as usize,
        WindowOptions::default())
        .unwrap();

    render(&mut image);

    let buf: &[u8] = &*image;
    let buf: &[u32] = unsafe { slice::from_raw_parts(buf.as_ptr() as _, buf.len() / 4) };

    window.update();
    while window.is_open() {
        window.update_with_buffer(buf, image.width() as _, image.height() as _).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    DynImage(image).into_rgb8().save("test.png").unwrap();
}
