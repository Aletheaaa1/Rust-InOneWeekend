pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::default();
    camera.render(&world);
}
