use image::{open, GrayImage};
use nanocv::{ImgBuf, ImgSize, filter::{map_new, vertical_filter, convolution_operator, update}};

fn main() {
    // Load image using piston image
    let buf = open("examples/raster.png").unwrap().into_luma();
    // Convert into ImgBuf
    let size = ImgSize::new(buf.width() as usize, buf.height() as usize);
    let img = ImgBuf::from_vec(size, buf.into_vec());
    // Convert to 16-bit image buffer
    let img = map_new(&img, |x| x as u16);
    let mut result = ImgBuf::new_like(&img);
    // Horizontal convolution filter
    let kernel = [1, 1, 1, 1, 1, 1, 1, 1, 1];
    vertical_filter(&img, &mut result, &kernel, convolution_operator);
    // Divide by 9 to fit into [0, 255] range
    update(&mut result, |x| x/9);
    // Convert back to 8-bit image 
    let result = map_new(&result, |x| x as u8);
    // Convert back to piston gray image
    let result = GrayImage::from_vec(size.x as u32, size.y as u32, result.into_vec()).unwrap();
    // Save result into target directory
    result.save("target/vertical_convolution.png").unwrap();
}