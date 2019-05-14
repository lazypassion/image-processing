extern crate image;

pub mod histogram;
pub mod integral_image;

use image::{RgbaImage};

pub fn get_mean(image: &RgbaImage) -> f64 {
    let image_iter = image.pixels();
    let mut mean: f64 = 0.0;
    for pixel in image_iter {
        mean += (f64::from(pixel[0]) + f64::from(pixel[1]) + f64::from(pixel[2])) / 3.0;
    }
    mean /= f64::from(image.width()) * f64::from(image.height());

    mean
}

pub fn get_variance(image: &RgbaImage) -> f64 {
    let mean = get_mean(&image);
    let mut variance: f64 = 0.0;
    let image_iter = image.pixels();
    for pixel in image_iter {
        let pixel_average = (f64::from(pixel[0]) + f64::from(pixel[1]) + f64::from(pixel[2])) / 3.0;
        variance += (pixel_average - mean).powi(2);
    }
    variance /= f64::from(image.width()) * f64::from(image.height());

    variance
}
