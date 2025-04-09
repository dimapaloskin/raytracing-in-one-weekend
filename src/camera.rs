use crate::{
    color::Color,
    math::{Point3, Vector3, degrees_to_radians, rand, vec3_rand_in_unit_disk},
};

use crate::{
    Buffer,
    hittable::HittableList,
    math::{Interval, Ray},
};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point3,
    zz_point: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    samples_per_pixel: u32,
    pixel_samples_scale: f32,
    max_ray_depth: i32,
    light_intensity: f32,
    sky_color: Color,
    defocus_angle: f32,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
}

pub struct CameraConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub aspect_ratio: f32,
    pub samples_per_pixel: u32,
    pub max_ray_depth: i32,
    pub light_intensity: f32,
    pub sky_color: Color,
    pub fov: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
}

impl Camera {
    pub fn new(config: &CameraConfig) -> Self {
        let theta = degrees_to_radians(config.fov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * config.focus_dist;
        let viewport_width: f32 = viewport_height * config.aspect_ratio;
        let center = config.look_from;

        let vup = Vector3::new(0.0, 1.0, 0.0);

        let w = (config.look_from - config.look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = u.cross(w);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * v;

        let pixel_delta_u = viewport_u / config.image_width as f32;
        let pixel_delta_v = viewport_v / config.image_height as f32;

        let viewport_upper_left =
            center - (config.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let zz_point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            config.focus_dist * degrees_to_radians(config.defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width: config.image_width,
            image_height: config.image_height,
            center,
            zz_point,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: config.samples_per_pixel,
            pixel_samples_scale: 1.0 / config.samples_per_pixel as f32,
            max_ray_depth: config.max_ray_depth,
            light_intensity: config.light_intensity,
            sky_color: config.sky_color,
            defocus_angle: config.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, bf: &mut Buffer, world: &HittableList) {
        for y in 0..self.image_height as usize {
            for x in 0..self.image_width as usize {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color += self.ray_color(&r, self.max_ray_depth, &world);
                }

                let pixel_color = pixel_color * self.pixel_samples_scale;
                bf.set_color(x, y, &pixel_color);
            }
        }
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.zz_point
            + ((x as f32 + offset.x) * self.pixel_delta_u)
            + ((y as f32 + offset.y) * self.pixel_delta_v);

        let ray_orig = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        Ray::new(ray_orig, pixel_sample - ray_orig)
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth < 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = world.hit(ray, &Interval::new(0.001, std::f32::INFINITY)) {
            if let Some((cl, scattered)) = hit.mat().scatter(ray, &hit) {
                let color = cl * self.ray_color(&scattered, depth - 1, world);
                return self.light_intensity * color;
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit = ray.dir().normalize();
        let a = 0.5 * (unit.y + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * self.sky_color
    }

    fn sample_square(&self) -> Vector3 {
        Vector3::new(rand() - 0.5, rand() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = vec3_rand_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
