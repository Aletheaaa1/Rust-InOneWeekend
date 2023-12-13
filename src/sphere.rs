use std::rc::Rc;

use crate::hittable;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        hit_record: &mut hittable::HitRecord,
    ) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root < ray_tmin || root > ray_tmax {
            root = (h + sqrtd) / a;
            if root < ray_tmin || root > ray_tmax {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        hit_record.mat = Some(Rc::clone(&self.mat));

        true
    }
}
