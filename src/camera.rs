use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    rtweekend::{self, random_double, random_double_range},
    vec3::{self, random_in_unit_sphere, random_on_hemisphere, random_unit_vector, Vec3},
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
    pub depth: i32,
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
            depth: 0,
        }
    }
}

impl Camera {
    pub fn initialize(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<i32>().unwrap();

        self.samplers = input;
        self.depth = 50;

        self.aspect_ratio = 16.0 / 9.0;
        self.image_width = 720;
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

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        let stdout = std::io::stdout();

        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let mut color = Vec3::default();
                for _ in 0..self.samplers {
                    let u =
                        (i as f64 + random_double_range(-1.0, 1.0)) / (self.image_width - 1) as f64;
                    let v = (j as f64 + random_double_range(-1.0, 1.0))
                        / (self.image_height - 1) as f64;

                    let dir = self.lower_left_corner + self.viewport_u * u + self.viewPort_v * v;
                    let r = Ray::new(self.center, dir);

                    color += self.ray_color(&r, world, self.depth);
                }

                color = color / self.samplers as f64;
                color.write_color(&mut stdout.lock());
            }
        }
    }

    pub fn ray_color(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        let mut hit_record = HitRecord::default();

        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if world.hit(r, 0.000001, 1000000.0, &mut hit_record) {
            let mut attenuation = Vec3::default();
            let mut scatter_ray = Ray::default();

            if let Some(mat) = hit_record.mat.clone() {
                if mat.scatter(&r, &hit_record, &mut attenuation, &mut scatter_ray) {
                    return attenuation * self.ray_color(&scatter_ray, world, depth - 1);
                }
            }
        }

        let unit_dir = vec3::unit_vector(r.direction());
        let mix = unit_dir.y() * 0.5 + 0.5;

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - mix) + Vec3::new(0.5, 0.7, 1.0) * mix
    }
}
