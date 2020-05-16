use image::{open, GrayImage};
use nanocv::{ImgBuf, ImgSize, filter::update};

fn main() {
    // Load image using piston image
    let buf = open("examples/raster.png").unwrap().into_luma();
    // Convert into ImgBuf
    let size = ImgSize::new(buf.width() as usize, buf.height() as usize);
    let mut img = ImgBuf::from_vec(size, buf.into_vec());
    // Compute negative image
    update(&mut img, |x| 255 - x);
    // Convert back to piston gray image
    let result = GrayImage::from_vec(size.x as u32, size.y as u32, img.into_vec()).unwrap();
    // Save result into target directory
    result.save("target/negative_image.png").unwrap();
}