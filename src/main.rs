pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

use std::rc::Rc;

use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn ray_color(r: &Ray, world: &mut HittableList) -> Color {
    let ball: Sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let mut rec = HitRecord::default();

    let t = world.hit(r, 0.000001, 100000.0, &mut rec);
    if t {
        let n = rec.normal * 0.5 + 0.5;
        return n;
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let a = 0.5 * unit_direction.y() + 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // Image
    let aspect_ration = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ration) as i32;

    // Camera
    let focus_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner =
        camera_center - Vec3::new(0.0, 0.0, focus_length) - (viewport_u + viewport_v) / 2.0;

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray_direction = lower_left_corner + viewport_u * u + viewport_v * v - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &mut world);
            pixel_color.write_color();
        }
    }
}
