#![feature(core)]

extern crate bmp;
use bmp::Image;
use std::num::Int;

fn main() {
  // 0 to 8
  for i in (0..9) {
    // Print the current step
    println!("{}", i);

    // Load the image, which is 512x512 large
    let img = bmp::open("img1.bmp").unwrap_or_else(|e| {
      panic!("Failed to open: {}", e);
    });

    // Rotate the image. Second argument will start from beind with 1, 2, 4, 8,
    // up to 256, where 1 provides the finest detail is the best finished
    // rotation, and 256 is just 1 blit level.
    let img_rotated = rotate(img, 2.pow(i));

    // Save the image
    let id = 10 - i;
    let name = format!("img{}.bmp", id);
    let _ = img_rotated.save(&name);
  }
}

fn rotate(img: Image, stop_at: u32) -> Image {
  if img.get_width() == stop_at {
    return img;
  }

  //  img1  |  img2
  //  -------------
  //  img3  |  img4
  let (img1, img2, img3, img4) = split(img);

  let img1_rotated = rotate(img1, stop_at);
  let img2_rotated = rotate(img2, stop_at);
  let img3_rotated = rotate(img3, stop_at);
  let img4_rotated = rotate(img4, stop_at);

  //  img3  |  img1
  //  -------------
  //  img4  |  img2
  combine(img3_rotated, img1_rotated, img4_rotated, img2_rotated)
}

fn split(img: Image) -> (Image, Image, Image, Image) {
  let width : u32 = img.get_width() / 2;
  let height : u32 = img.get_height() / 2;
  let (mut img1, mut img2, mut img3, mut img4) = (
    bmp::Image::new(width, height),
    bmp::Image::new(width, height),
    bmp::Image::new(width, height),
    bmp::Image::new(width, height),
  );
  for (x, y) in img.coordinates() {
    let pixel = img.get_pixel(x, y);
    if x < width && y < height {
      img1.set_pixel(x, y, pixel);
    } else if x >= width && y < height {
      img2.set_pixel(x - width, y, pixel);
    } else if x < width && y >= height {
      img3.set_pixel(x, y - height, pixel);
    } else if x >= width && y >= height {
      img4.set_pixel(x - width, y - height, pixel);
    }
  }
  (img1, img2, img3, img4)
}

fn combine(img1: Image, img2: Image, img3: Image, img4: Image) -> Image {
  let width : u32 = img1.get_width();
  let height : u32 = img1.get_height();
  let mut img_new = bmp::Image::new(width * 2, height * 2);
  for x in (0..width) {
    for y in (0..height) {
      img_new.set_pixel(x, y, img1.get_pixel(x, y));
      img_new.set_pixel(x + width, y, img2.get_pixel(x, y));
      img_new.set_pixel(x, y + height, img3.get_pixel(x, y));
      img_new.set_pixel(x + width, y + height, img4.get_pixel(x, y));
    }
  }
  img_new
}
