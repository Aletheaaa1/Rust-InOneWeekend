use crate::vec3;

pub type Color = vec3::Vec3;

impl Color {
    pub fn write_color(&self, stdout: &mut dyn std::io::Write) {
        writeln!(
            stdout,
            "{} {} {}",
            (255.999 * self.x().powf(1.0 / 2.2)) as i32,
            (255.999 * self.y().powf(1.0 / 2.2)) as i32,
            (255.999 * self.z().powf(1.0 / 2.2)) as i32
        )
        .unwrap();
    }
}
