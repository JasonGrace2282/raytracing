use indicatif::{ProgressBar, ProgressStyle};
use raytrace::{
    hit::{Hittable, HittableList},
    sphere::Sphere,
    utils::{write_color, Color, Interval, Point, Ray, Rc, Vec3},
};

fn ray_color(world: &HittableList<f64>, ray: &Ray<f64>) -> Color<f64> {
    let interval = Interval::new(0.0, f64::INFINITY);
    if let Some(rec) = world.hit(ray, interval) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = ray.get_direction().unit_vector();
    let a = (unit_direction.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let mut image_height = image_width / aspect_ratio;
    if image_height < 1.0 {
        image_height = 1.0;
    }

    let mut world: HittableList<f64> = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera setup
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // viewport stuff
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate location of top left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel100_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let progress = ProgressBar::new(image_height as u64);
    progress.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );

    // Render
    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height as i32 {
        progress.inc(1);
        for i in 0..image_width as i32 {
            let pixel_center =
                pixel100_loc + (pixel_delta_u * (i as f64)) + (pixel_delta_v * (j as f64));
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&world, &ray);

            write_color(color)
        }
    }
    progress.finish();
}
