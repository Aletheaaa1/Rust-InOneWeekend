use std::io::Write;

use crate::vec3;

pub type Color = vec3::Vec3;

impl Color {
    pub fn write_color(&self, out: &mut dyn Write) {
        write!(
            out,
            "{} {} {}\n",
            (255.999 * self.x()) as i32,
            (255.999 * self.y()) as i32,
            (255.999 * self.z()) as i32
        )
        .unwrap();
    }
}
