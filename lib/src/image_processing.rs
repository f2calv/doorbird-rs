use std::time::Instant;

use image::{imageops, GenericImageView, ImageError};

pub struct ProcessImage {
    pub test: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl ProcessImage {
    pub fn new(test: String) -> Self {
        Self {
            test,
            x: 100,
            y: 100,
            width: 600,
            height: 1100,
        }
    }

    pub fn run_image_tests(&self) -> Result<bool, ImageError> {
        let start = Instant::now();

        // Use the open function to load an image from a Path.
        // `open` returns a `DynamicImage` on success.
        let mut img = image::open("/testdata/test-1920x1440.jpg")?;

        // The dimensions method returns the images width and height.
        println!("img dimensions {:?}", img.dimensions());

        img = img.crop(self.x, self.y, self.width, self.height);
        println!("img dimensions2 {:?}", img.dimensions());
        img.save("/testdata/cropped.jpg")?;

        // let cropped = imageops::crop(&mut img, 1000, 1000, 1000, 1000);
        // println!("cropped dimensions {:?}", cropped.dimensions());
        // cropped.to_image().save("/testdata/cropped.jpg")?;

        //let rotated = imageops::rotate180(cropped);

        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());

        // Write the contents of this image to the Writer in PNG format.
        //img.save("/testdata/test.png")?;

        let duration = start.elapsed();
        println!("Time elapsed is: {:?}", duration);

        //Note: test with 'cargo run --release' !

        Ok(true)
    }
}
