extern crate image; 

use image::{ImageBuffer, RgbaImage, GrayAlphaImage, ConvertBuffer, LumaA, Pixel, GrayImage};

struct RgbHistogram {
    red: [u32; 256],
    green: [u32; 256],
    blue: [u32; 256],
}

struct LumaHistogram {
    values: [u32; 256],
}

impl RgbHistogram {
    fn new() -> RgbHistogram{
        RgbHistogram {
            red: [0; 256],
            green: [0; 256],
            blue: [0; 256],
        }
    }
}

impl LumaHistogram {
    fn new() -> LumaHistogram{
     LumaHistogram {
            values: [0; 256], 
        }
    }
}

fn main() {

    let img: GrayAlphaImage = image::open("images/london-bridge.jpg")
        .expect("Image not found").to_luma_alpha();

    let mut image: GrayAlphaImage = ImageBuffer::new(2, 2);

    // for pix in image.pixels_mut() {
    //     let channels = pix.channels_mut();
    //     channels[0] = 100;
    //     channels[1] = 255;  
    // }

    // let hist = gray_histogram(&image);
    // for i in 0..255 {
    //     println!("{}: {}", i,  &hist.values[i]);
    // }
    let hist = lumaA_histogram(&img);

    // img.save("images/gray-london-bridge.jpg").expect("directory or file not found");
    

    // let mean = get_mean(&img);
    // let variance = get_variance(&img);

    // println!("mean: {}\nvariance: {}", mean, variance); 
}

fn rgba_histogram(image: &RgbaImage) -> RgbHistogram {
    let mut histogram = RgbHistogram::new();
    for pixel in image.pixels() {
        histogram.red[pixel[0] as usize] += 1;
        histogram.green[pixel[1] as usize] += 1;
        histogram.blue[pixel[2] as usize] += 1;
    }
    histogram
}

fn rgb_histogram(image: &RgbaImage) -> RgbHistogram {
    let mut histogram = RgbHistogram::new();
    for pixel in image.pixels() {
        histogram.red[pixel[0] as usize] += 1;
        histogram.green[pixel[1] as usize] += 1;
        histogram.blue[pixel[2] as usize] += 1;
    }
    histogram
}

fn lumaA_histogram(image: &GrayAlphaImage) -> LumaHistogram {
    let mut histogram = LumaHistogram::new();
    for pixel in image.pixels() {
        histogram.values[pixel[0] as usize] += 1;
    }
    histogram
}

fn luma_histogram(image: &GrayImage) -> LumaHistogram {
    let mut histogram = LumaHistogram::new();
    for pixel in image.pixels() {
        histogram.values[pixel[0] as usize] += 1;
    }
    histogram
}

fn get_mean(image: &RgbaImage) -> f64 {
    let image_iter = image.pixels(); 
    let mut mean: f64 = 0.0;
    for pixel in image_iter {
        mean += (f64::from(pixel[0]) + f64::from(pixel[1]) + 
                f64::from(pixel[2])) / 3.0; 
    }
    mean /= f64::from(image.width()) * f64::from(image.height()); 

    mean
}

fn get_variance(image: &RgbaImage) -> f64 {
    let mean = get_mean(&image); 
    let mut variance: f64 = 0.0;
    let image_iter = image.pixels(); 
    for pixel in image_iter {
        let pixel_average = (f64::from(pixel[0]) + f64::from(pixel[1]) + 
                f64::from(pixel[2])) / 3.0;
        variance += (pixel_average - mean).powi(2); 
    }
    variance /= f64::from(image.width()) * f64::from(image.height());
    
    variance
}


#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImage, ImageBuffer, GenericImageView, RgbaImage, GrayAlphaImage, ConvertBuffer, GrayImage, Luma, Rgb, load_from_memory};
    #[test]
    fn test_histogram() {
        let mut image: GrayAlphaImage = ImageBuffer::new(2, 2);

        image.get_pixel_mut(0, 0)[0] = 1; 
        image.get_pixel_mut(0, 1)[0] = 1; 
        image.get_pixel_mut(1, 0)[0] = 3; 
        image.get_pixel_mut(1, 1)[0] = 4; 

        let hist = lumaA_histogram(&image);

        assert_eq!(hist.values[1], 2);
        assert_eq!(hist.values[3], 1);
        assert_eq!(hist.values[4], 1);
    }
}