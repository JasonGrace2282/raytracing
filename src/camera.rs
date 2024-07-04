use crate::{
    hit::{Hittable, HittableList},
    utils::{rand_float, write_color, Color, Interval, Point, Ray, Vec3, rand_in_unit_disk},
};
use indicatif::{ProgressBar, ProgressStyle};
use num_traits::Float;

#[derive(Debug)]
pub struct Camera {
    image_width: f64,
    samples_per_pixel: i32,
    sample_pixel_scale: f64,
    image_height: f64,
    center: Point<f64>,
    pixel100_loc: Point<f64>,
    pixel_delta_u: Vec3<f64>,
    pixel_delta_v: Vec3<f64>,
    defocus_disc_u: Vec3<f64>,
    defocus_disc_v: Vec3<f64>,
    defocus_angle: f64,
    max_depth: i32,
    progress: ProgressBar,
}

impl Default for Camera {
    fn default() -> Self {
        // hardcoded
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400.0;

        let samples_per_pixel = 100;

        let max_depth = 50;

        let vfov = 20.0;

        let center = Point::new(-2.0, 2.0, 1.0); // where camera is located
        let lookat = Point::new(-0.0, -0.0, -1.0); // where it's looking
        let vup = Vec3::new(0.0, 1.0, 0.0); // the direction up relative to the camera

        let defocus_angle = 10.0;
        let focus_dist = 3.4;

        Self::new(
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            center,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
        )
    }
}

#[allow(clippy::too_many_arguments)]
impl Camera {
    fn new(
        aspect_ratio: f64,
        image_width: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        center: Vec3<f64>, // lookfrom
        lookat: Vec3<f64>,
        vup: Vec3<f64>,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        // computed stuff goes here
        let mut image_height = image_width / aspect_ratio;
        if image_height < 1.0 {
            image_height = 1.0;
        }

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (center - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left = center - (w * focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel100_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        let sample_pixel_scale = 1.0 / samples_per_pixel as f64;

        let progress = ProgressBar::new(image_height as u64);
        progress.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );

        Self {
            image_width,
            samples_per_pixel,
            sample_pixel_scale,
            image_height,
            center,
            pixel100_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disc_u,
            defocus_disc_v,
            defocus_angle,
            max_depth,
            progress,
        }
    }

    pub fn render(&self, world: HittableList<f64>) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height as i32 {
            self.progress.inc(1);
            for i in 0..self.image_width as i32 {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += Self::ray_color(&world, ray, self.max_depth);
                }
                write_color(color * self.sample_pixel_scale)
            }
        }

        self.progress.finish();
    }

    fn ray_color(world: &HittableList<f64>, ray: Ray<f64>, depth: i32) -> Color<f64> {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let interval = Interval::new(0.000000001, f64::INFINITY);
        if let Some(rec) = world.hit(&ray, interval) {
            if let Some((attenuation, scattered)) = rec.material.scatter(&ray, &rec) {
                let ray_color = Self::ray_color(world, scattered, depth - 1);
                return attenuation.mul_vec3(ray_color);
            }
            return Color::new(0.0, 0.0, 0.0);
        }
        let unit_direction = ray.get_direction().unit_vector();
        let a = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray<f64> {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel100_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3<f64> {
        let p = rand_in_unit_disk();
        self.center + (self.defocus_disc_u * p.x) + (self.defocus_disc_v * p.y)
    }

    fn sample_square() -> Vec3<f64> {
        Vec3::new(rand_float() - 0.5, rand_float() - 0.5, 0.0)
    }
}
