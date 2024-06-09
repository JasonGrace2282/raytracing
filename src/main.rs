use raytrace::{
    camera::Camera,
    hit::HittableList,
    sphere::Sphere,
    utils::{Point, Rc},
};

fn main() {
    let mut world: HittableList<f64> = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::default();

    camera.render(world);
}
