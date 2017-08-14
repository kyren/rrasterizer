use num;

use vec4::Vector4;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

pub fn vec4_to_color(Vector4 { x, y, z, w }: Vector4<f32>) -> Color {
    Color(
        num::clamp(x * 255.0, 0.0, 255.0) as u8,
        num::clamp(y * 255.0, 0.0, 255.0) as u8,
        num::clamp(z * 255.0, 0.0, 255.0) as u8,
        num::clamp(w * 255.0, 0.0, 255.0) as u8,
    )
}
