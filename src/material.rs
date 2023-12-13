use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{self, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scatter_ray: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: &Vec3) -> Self {
        Lambertian { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scatter_ray: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + vec3::random_unit_vector();
        *scatter_ray = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}
