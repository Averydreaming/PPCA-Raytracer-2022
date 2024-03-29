use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::{MovingSphere, Sphere};
use crate::texture::{Checker_Texture, Noise_Texture};
use crate::vec3::{Point3, RGBColor, Vec3};
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{
    fs::File,
    process::exit,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};
pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod sphere;
pub mod texture;
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
    let mut world = HittableList::new(); /*
                                         let ground_material = Arc::new(Lambertian::new(RGBColor::new(0.5, 0.5, 0.5)));
                                         world.add(Arc::new(Sphere::new(
                                             Point3::new(0., -1000., 0.),
                                             1000.,
                                             ground_material,
                                         )));
                                         */
    let checker = Arc::new(Checker_Texture::new(
        RGBColor::new(0.2, 0.3, 0.1),
        RGBColor::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_arc(checker)),
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
                    let center2 = center
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
fn Two_Spheres() -> HittableList {
    let mut objects = HittableList::new();
    /*let checker = Arc::new(Checker_Texture::new(
        RGBColor::new(0.2, 0.3, 0.1),
        RGBColor::new(0.9, 0.9, 0.9),
    ));
    */
    let pertext = Arc::new(Noise_Texture::new());
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_arc(pertext.clone())),
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::new_arc(pertext)),
    )));
    return objects;
}
fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    // Image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = 225;
    const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    const IMAGE_QUALITY: u8 = 100; // From 0 to 100
    let path = "output/output.jpg";
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    const THREAD_NUMBER: u32 = 8;
    const SECTION_LINE_NUM: u32 = IMAGE_HEIGHT / THREAD_NUMBER;

    println!(
        "Image size: {}\nJPEG IMAGE_QUALITY: {}",
        style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow(),
        style(IMAGE_QUALITY.to_string()).yellow(),
    );

    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.,
        ASPECT_RATIO,
        0.0,
        10.,
        0.,
        1.,
    );

    // Progress bar
    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true);

    // Thread
    let mut output_pixel_color = Vec::<RGBColor>::new();
    let mut thread_pool = Vec::<_>::new();

    // World
    // let main_world = random_scene();
    //let main_world = BvhNode::new_list(&random_scene(), 0., 1.);
    let main_world = BvhNode::new_list(&Two_Spheres(), 0., 1.);
    for thread_id in 0..THREAD_NUMBER {
        // line
        let line_beg = thread_id * SECTION_LINE_NUM;
        let mut line_end = line_beg + SECTION_LINE_NUM;
        if thread_id == THREAD_NUMBER - 1 {
            line_end = IMAGE_HEIGHT;
        }

        // world
        let world = main_world.clone();

        //progress
        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new((line_end - line_beg) as u64));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        // thread code
        let (tx, rx) = mpsc::channel();

        thread_pool.push((
            thread::spawn(move || {
                let mut progress = 0;
                progress_bar.set_position(progress);

                let mut section_pixel_color = Vec::<RGBColor>::new();

                let mut rng = rand::thread_rng();
                for y in line_beg..line_end {
                    for x in 0..IMAGE_WIDTH {
                        let mut pixel_color = RGBColor::new(0., 0., 0.);
                        for _i in 0..SAMPLES_PER_PIXEL {
                            let rand_u: f64 = rng.gen();
                            let rand_v: f64 = rng.gen();
                            let u = (x as f64 + rand_u) / (IMAGE_WIDTH - 1) as f64;
                            let v = (y as f64 + rand_v) / (IMAGE_HEIGHT - 1) as f64;
                            let r = cam.get_ray(u, v);
                            pixel_color += Ray::ray_color(r, &world, MAX_DEPTH);
                        }
                        section_pixel_color.push(pixel_color);
                    }
                    progress += 1;
                    progress_bar.set_position(progress);
                }
                tx.send(section_pixel_color).unwrap();
                progress_bar.finish_with_message("Finished.");
            }),
            rx,
        ));
    }
    multiprogress.join().unwrap();

    let mut thread_finish = true;
    for _thread_id in 0..THREAD_NUMBER {
        let thread = thread_pool.remove(0);
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
            }
            Err(_) => {
                thread_finish = false;
            }
        }
    }

    if !thread_finish {
        println!("run time error");
        exit(0);
    }

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut pixel_id = 0;
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let pixel_color = output_pixel_color[pixel_id];
            let pixel = img.get_pixel_mut(x, IMAGE_HEIGHT - y - 1);
            *pixel = image::Rgb(write_color(pixel_color, SAMPLES_PER_PIXEL));
            pixel_id += 1;
        }
    }

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(
        &mut output_file,
        image::ImageOutputFormat::Jpeg(IMAGE_QUALITY),
    ) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
