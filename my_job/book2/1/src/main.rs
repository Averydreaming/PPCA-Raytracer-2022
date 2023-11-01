use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::sphere::{MovingSphere, Sphere};
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
fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(RGBColor::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    let mut rng = rand::thread_rng();
    for a in -11..10 {
        for b in -11..10 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mut rng1 = rand::thread_rng();
                    let albedo = RGBColor::random(0., 1.) * RGBColor::random(0., 1.);
                    let mut center2 = center
                        + Vec3 {
                            x: (0.),
                            y: (rng1.gen_range(0.0..0.5)),
                            z: (0.0),
                        };
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = RGBColor::random(0.5, 1.);
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        Arc::new(Dielectric::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        Arc::new(Lambertian::new(RGBColor::new(0.4, 0.2, 0.1))),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        Arc::new(Metal::new(RGBColor::new(0.7, 0.6, 0.5), 0.)),
    )));

    return world;
}
fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let height = 225;
    let width = 400;
    let aspect_ratio = 16.0 / 9.0;
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
    let world = random_scene();
    //camera
    let cam = Camera::new(
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );
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
