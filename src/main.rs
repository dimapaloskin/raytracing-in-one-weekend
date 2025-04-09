use chrono::{DateTime, Local};
use image::{ImageFormat, Rgba, RgbaImage};
use minifb::Window;
use raytracer::camera::{Camera, CameraConfig};
use raytracer::color::Color;
use raytracer::hittable::HittableList;
use raytracer::hittable::sphere::Sphere;
use raytracer::materials::dielectric::{Dielectric, DielectricConfig};
use raytracer::materials::lambertian::LambertianConfig;
use raytracer::materials::metal::MetalConfig;
use raytracer::materials::{lambertian::Lambertian, metal::Metal};
use raytracer::math::{Point3, rand, rand_rng};
use raytracer::{App, Buffer};
use std::rc::Rc;
use std::time::SystemTime;

const WIDTH: f32 = 800.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: f32 = WIDTH / ASPECT_RATIO;

struct State {
    camera_config: CameraConfig,
}

fn main() {
    let camera_config = CameraConfig {
        image_width: WIDTH as u32,
        image_height: HEIGHT as u32,
        aspect_ratio: ASPECT_RATIO,
        samples_per_pixel: 2500,
        max_ray_depth: 200,
        light_intensity: 0.95,
        sky_color: Color::new(0.5, 0.7, 1.0),

        fov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,
    };
    let mut state = State { camera_config };

    let mut app = App::create(WIDTH as usize, HEIGHT as usize, &mut state, Some(on_init)).unwrap();
    app.set_fps(120).run_with_callback(render_callback);
}

fn render_callback(_bf: &mut Buffer, _window: &Window, _state: &mut State) {}

fn on_init(bf: &mut Buffer, _window: &Window, state: &mut State) {
    let cam = Camera::new(&state.camera_config);
    let mut world = HittableList::new();

    fill_world(&mut world);

    let start = SystemTime::now();
    let datetime = DateTime::<Local>::from(start);
    println!("Started at {}", datetime.format("%Y-%m-%d %H:%M:%S.%3f"));

    cam.render(bf, &world);

    let finish = SystemTime::now();

    save_image(bf);

    let datetime = DateTime::<Local>::from(finish);
    let duration = finish.duration_since(start).unwrap();

    println!("Finished at {}", datetime.format("%Y-%m-%d %H:%M:%S.%3f"));
    println!("Render time: {:.3}m", duration.as_secs_f64() / 60.0);
}

fn fill_world(world: &mut HittableList) {
    let ground_mat = Rc::new(Lambertian::new(LambertianConfig {
        albedo: Color::new(0.73, 0.8, 0.87),
    }));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&ground_mat),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let r_mat = rand();
            let center = Point3::new(a as f32 + 0.9 * rand(), 0.2, b as f32 + 0.9 * rand());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if r_mat < 0.8 {
                    let albedo =
                        Color::new(rand(), rand(), rand()) * Color::new(rand(), rand(), rand());

                    let mat = Rc::new(Lambertian::new(LambertianConfig { albedo }));
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::clone(&mat))));
                } else if r_mat < 0.95 {
                    let albedo =
                        Color::new(rand(), rand(), rand()) * Color::new(rand(), rand(), rand());
                    let fuzz = rand_rng(0.0, 0.5);

                    let mat = Rc::new(Metal::new(MetalConfig { albedo, fuzz }));
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::clone(&mat))));
                } else {
                    let mat = Rc::new(Dielectric::new(DielectricConfig {
                        refraction_index: 1.5,
                    }));
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::clone(&mat))));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(DielectricConfig {
        refraction_index: 1.5,
    }));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::clone(&mat1),
    )));

    let mat2 = Rc::new(Lambertian::new(LambertianConfig {
        albedo: Color::new(0.4, 0.2, 0.1),
    }));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&mat2),
    )));

    let mat3 = Rc::new(Metal::new(MetalConfig {
        albedo: Color::new(0.7, 0.7, 0.5),
        fuzz: 0.0,
    }));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&mat3),
    )));
}

fn save_image(bf: &Buffer) {
    let mut img = RgbaImage::new(WIDTH as u32, HEIGHT as u32);
    for y in 0..HEIGHT as u32 {
        for x in 0..WIDTH as u32 {
            let idx = (y * WIDTH as u32 + x) as usize;
            let pixel_value = bf.buffer()[idx];

            let r = ((pixel_value >> 16) & 0xFF) as u8;
            let g = ((pixel_value >> 8) & 0xFF) as u8;
            let b = (pixel_value & 0xFF) as u8;
            let a = ((pixel_value >> 24) & 0xFF) as u8;

            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    img.save_with_format("last_render.png", ImageFormat::Png)
        .unwrap();
}
