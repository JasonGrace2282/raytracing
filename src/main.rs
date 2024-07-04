use raytrace::{
    camera::Camera,
    hit::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
    utils::{rand_float, rand_from_range, Color, Point, Rc},
};

fn main() {
    let mut world: HittableList<f64> = HittableList::new();

    let ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_float();
            let center = Point::new(
                a as f64 + 0.9 * rand_float(),
                0.2,
                b as f64 + 0.9 * rand_float(),
            );
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material<f64>> = {
                    if choose_mat < 0.8 {
                        let albedo = Color::<f64>::rand().mul_vec3(Color::<f64>::rand());
                        Rc::new(Lambertian::new(albedo))
                    } else if choose_mat < 0.95 {
                        let albedo = Color::<f64>::rand_from_range(0.5, 1.0);
                        let fuzziness = rand_from_range(0.0..0.5);
                        Rc::new(Metal::new(albedo, Some(fuzziness)))
                    } else {
                        Rc::new(Dielectric::new(1.5))
                    }
                };
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), Some(0.0)));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let camera = Camera::default();

    camera.render(world);
}
