use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    rtweekend::{self, random_double},
    vec3::{self, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub center: Vec3,
    pub viewport_u: Vec3,
    pub viewPort_v: Vec3,
    pub lower_left_corner: Vec3,
    pub samplers: i32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Vec3::default(),
            viewport_u: Vec3::default(),
            viewPort_v: Vec3::default(),
            lower_left_corner: Vec3::default(),
            samplers: 0,
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        let mut stdout = std::io::stdout();

        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let mut color = Vec3::default();
                for _ in 0..self.samplers {
                    let u = (i as f64 + random_double()) / (self.image_width - 1) as f64;
                    let v = (j as f64 + random_double()) / (self.image_height - 1) as f64;

                    let dir = self.lower_left_corner + self.viewport_u * u + self.viewPort_v * v;
                    let r = Ray::new(self.center, dir);

                    color += self.ray_color(&r, world);
                }

                color = color / self.samplers as f64;
                color.write_color(&mut stdout.lock());
            }
        }
    }

    pub fn initialize(&mut self) {
        self.samplers = 10;

        self.aspect_ratio = 16.0 / 9.0;
        self.image_width = 512;
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;

        let focus_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        self.viewPort_v = Vec3::new(0.0, viewport_height, 0.0);

        self.lower_left_corner = self.center
            - (self.viewPort_v + self.viewport_u) / 2.0
            - Vec3::new(0.0, 0.0, focus_length);
    }

    pub fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Vec3 {
        let mut hit_record = HitRecord::default();

        if world.hit(r, 0.000001, 1000000.0, &mut hit_record) {
            return hit_record.normal * 0.5 + 0.5;
        }

        let unit_dir = vec3::unit_vector(r.direction());
        let mix = unit_dir.y() * 0.5 + 0.5;

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - mix) + Vec3::new(0.5, 0.7, 1.0) * mix
    }
}
