use crate::{
    hit::{Hittable, HittableList},
    utils::{rand_float, random_on_hemisphere, write_color, Color, Interval, Point, Ray, Vec3},
};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f64,
    image_width: f64,
    samples_per_pixel: i32,
    sample_pixel_scale: f64,
    image_height: f64,
    center: Point<f64>,
    pixel100_loc: Point<f64>,
    pixel_delta_u: Vec3<f64>,
    pixel_delta_v: Vec3<f64>,
    max_depth: i32,
    progress: ProgressBar,
}

impl Default for Camera {
    fn default() -> Self {
        // hardcoded
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400.0;

        let focal_length = 1.0;
        let viewport_height = 2.0;

        let samples_per_pixel = 10;

        let max_depth = 10;

        // computed stuff goes here
        let mut image_height = image_width / aspect_ratio;
        if image_height < 1.0 {
            image_height = 1.0;
        }

        let viewport_width = viewport_height * (image_width / image_height);
        let center = Vec3::new(0.0, 0.0, 0.0);

        // viewport stuff
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel100_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let sample_pixel_scale = 1.0 / samples_per_pixel as f64;

        let progress = ProgressBar::new(image_height as u64);
        progress.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            sample_pixel_scale,
            image_height,
            center,
            pixel100_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_depth,
            progress,
        }
    }
}

impl Camera {
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
            let direction = random_on_hemisphere(&rec.normal);
            return Self::ray_color(world, Ray::new(rec.point, direction), depth-1) * 0.5;
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

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3<f64> {
        Vec3::new(rand_float() - 0.5, rand_float() - 0.5, 0.0)
    }
}
