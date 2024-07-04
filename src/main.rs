use raytrace::{
    camera::Camera,
    hit::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    utils::{Color, Point, Rc},
};

fn main() {
    let mut world: HittableList<f64> = HittableList::new();

    let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left = Rc::new(Dielectric::new(1.0 / 1.33));
    let right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), Some(1.0)));

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        center,
    )));
    world.add(Rc::new(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left)));
    world.add(Rc::new(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right)));

    let camera = Camera::default();

    camera.render(world);
}
