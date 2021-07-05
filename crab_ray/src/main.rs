use std::{slice, thread, time::Duration};
use image::{Bgra as Pixel, DynamicImage::ImageBgra8 as DynImage, ImageBuffer};
use minifb::{Window, WindowOptions};

mod raytracer;
use raytracer::{Renderer, make_scene};

type Image = ImageBuffer::<Pixel<u8>, Vec<u8>>;

fn main() {
    let mut renderer = Renderer::new(make_scene());

    let mut window = Window::new(
        "crab rayve",
        renderer.image.width() as usize,
        renderer.image.height() as usize,
        WindowOptions::default())
        .unwrap();

    renderer.render();

    let image: &Image = &renderer.image;
    let buf: &[u8] = &*image;
    let buf: &[u32] = unsafe { slice::from_raw_parts(buf.as_ptr() as _, buf.len() / 4) };

    window.update();
    while window.is_open() {
        window.update_with_buffer(buf, image.width() as _, image.height() as _).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    std::fs::create_dir_all("output").expect("failed to create output/");
    DynImage(renderer.image).into_rgb8().save("output/test.png").unwrap();
}
