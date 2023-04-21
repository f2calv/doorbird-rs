use image::{GenericImageView, ImageError};

pub struct ProcessImage {
    pub test: String,
}

impl ProcessImage {
    pub fn new(test: String) -> Self {
        Self { test }
    }

    pub fn run_image_tests(&self) -> Result<bool, ImageError> {
        // Use the open function to load an image from a Path.
        // `open` returns a `DynamicImage` on success.
        let img = image::open("tests/test.jpg")?;

        // The dimensions method returns the images width and height.
        println!("dimensions {:?}", img.dimensions());

        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());

        // Write the contents of this image to the Writer in PNG format.
        img.save("test.png")?;

        Ok(true)
    }
}
