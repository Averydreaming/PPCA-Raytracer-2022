use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, RGBColor, Vec3};
use console::style;
use core::f64::consts::PI;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::{fs::File, process::exit};
pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3; //调用模块
use rand::Rng;
fn write_color(pixel_color: RGBColor, samples_per_pixel: i32) -> [u8; 3] {
    [
        /*let mut r=pixel_color.x;
        let mut g=pixel_color.y;
        let mut b=pixel_color.z;
        let */
        ((pixel_color.x / samples_per_pixel as f64)
            .sqrt()
            .clamp(0.0, 0.999)
            * 255.999)
            .floor() as u8,
        ((pixel_color.y / samples_per_pixel as f64)
            .sqrt()
            .clamp(0.0, 0.999)
            * 255.999)
            .floor() as u8,
        ((pixel_color.z / samples_per_pixel as f64)
            .sqrt()
            .clamp(0.0, 0.999)
            * 255.999)
            .floor() as u8,
    ]
}
fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1
    let aspect_ratio = 16.0 / 9.0;
    let height = 225;
    let width = 400;
    let quality = 100; // From 0 to 100
    let samples_per_pixel = 100;
    let max_depth = 50.0;
    let path = "output/output.jpg";
    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + &"x".to_string() + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );
    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));
    //World
    let R = (PI / 4.0).cos();
    let mut world: HittableList = HittableList::new();
    let material_left = Arc::new(Lambertian::new(RGBColor::new(0.0, 0.0, 1.0)));
    let material_right = Arc::new(Lambertian::new(RGBColor::new(1.0, 0.0, 0.0)));

    world.add(Arc::new(Sphere::new(
        Point3::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(R, 0.0, -1.0),
        R,
        material_right,
    )));
    //camera
    let cam = Camera::new(90.0, aspect_ratio);
    // Generate image
    for y in 0..height {
        for x in 0..width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            let mut rnd = rand::thread_rng();
            for s in 0..samples_per_pixel {
                //   let mut _u=
                let u = (x as f64 + rnd.gen::<f64>()) / (width - 1) as f64;
                let v = (y as f64 + rnd.gen::<f64>()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + Ray::ray_color(r, &world, max_depth);
            }
            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(write_color(pixel_color, 100));
            progress.inc(1);
        }
    }
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
