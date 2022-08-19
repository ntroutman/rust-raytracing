extern crate core;

use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use mediumvec::{Vec32, vec32};

use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;

mod ray;
mod vec3;

fn main() {
    let img = Image::gradiant(1280, 720);
    //img.write_targa(String::from("colors.tga"));

    let  aspect_ratio = img.width as f32 / img.height as f32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0, 0, 0);
    let horizontal = Vec3::from(viewport_width, 0, 0);
    let vertical = Vec3::from(0, viewport_height, 0);
    let lower_left_corner = origin - horizontal/2 - vertical/2 - Vec3::from(0, 0, focal_length);

}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::from(1.0, 1.0, 1.0) + (t * Color::from(0.5, 0.7, 1.0));
}

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Image {
    pub fn from(width: usize, height: usize) -> Image {
        let size = (3 as usize * width as usize * height as usize);
        let pixels = vec![0; size]; // Vec::with_capacity(size);
        return Image { width, height, pixels };
    }

    pub fn pixels(&mut self, x: usize, y: usize, clr: Color) {
        let idx = self.index(x, y);

        if let Some(pix) = self.pixels.get_mut(idx..idx + 3) {
            pix[0] = (255.0 * clr.x()) as u8;
            pix[1] = (255.0 * clr.y()) as u8;
            pix[2] = (255.0 * clr.z()) as u8;
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        return 3 * (y * self.width + x);
    }

    pub fn gradiant(width: usize, height: usize) -> Image {
        let mut img = Image::from(width, height);

        for y in 0..img.height {
            for x in 0..img.width {
                let r = (x as f32) / (width - 1) as f32;
                let g = (y as f32) / (height - 1) as f32;
                let b = 0.25;

                img.pixels(x, y, Color::from(r, g, b));
            }
        }

        return img;
    }

    pub fn write_targa(&self, filename: String) {
        let mut file = match File::create(Path::new(&filename)) {
            Err(why) => panic!("couldn't create {}: {}", filename, why),
            Ok(file) => file,
        };

        let image_width = self.width.to_le_bytes();
        let image_height = self.height.to_le_bytes();
        // Format
        let header = [
            // 1 	1 byte 	ID length 	Length of the image ID field
            0,
            // 2 	1 byte 	Color map type 	Whether a color map is included
            0,
            // 3 	1 byte 	Image type 	Compression and color types
            2, // Raw image, no color map
            // 4 	5 bytes 	Color map specification 	Describes the color map
            0, 0, 0, 0, 0,
            // 5 	10 bytes 	Image specification 	Image dimensions and format
            // - 2 bytes    X origin
            0, 0,
            // - 2 bytes    Y origin
            0, 0,
            // - 2 bytes    Width
            image_width[0], image_width[1],
            // - 2 bytes    Width
            image_height[0], image_height[1],
            // - 1 byte     Pixel Depth (bits per Pixel)
            24,
            // - 1 byte     Image Descriptor
            1 << 5 // Bit 5 = top-to-bottom ordering
        ];
        file.write(&header);

        // 6 	From image ID length field 	Image ID 	Optional field containing identifying information
        // 7 	From color map specification field 	Color map data 	Look-up table containing color map data
        // 8 	From image specification field 	Image data 	Stored according to the image descriptor
        file.write(self.pixels.borrow());
    }
}
