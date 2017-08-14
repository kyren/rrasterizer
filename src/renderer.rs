use vec2::Vector2;
use vec3::Vector3;
use vec4::Vector4;
use mat4::Matrix4;
use bound_rect::BoundRect;
use color::{Color, vec4_to_color};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Vector4<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

pub struct Renderer {
    dimensions: (u32, u32),
    framebuffer: Vec<Color>,
    perspective: Matrix4<f32>,
}

impl Renderer {
    pub fn new(width: u32, height: u32, fov: f32) -> Renderer {
        let scale = (fov * 0.5).tan();
        let perspective = Matrix4::perspective(scale * 2.0, scale * 2.0, -1.0, 1.0);
        Renderer {
            dimensions: (width, height),
            framebuffer: vec![Color(0, 0, 0, 255); (width * height) as usize],
            perspective,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn clear(&mut self, color: Color) {
        for p in &mut self.framebuffer {
            *p = color;
        }
    }

    pub fn render(&mut self, transformation: Matrix4<f32>, triangles: &[Triangle]) {
        let screen_bounds =
            BoundRect::from_bounds(0.0, 0.0, self.dimensions.0 as f32, self.dimensions.1 as f32);

        let transformation = self.perspective * transformation;
        let (width, height) = self.dimensions;
        let screen_transform = move |v| {
            let v = transformation.transform(v);
            Vector3::new(
                (v.x + 1.0) / 2.0 * width as f32,
                (v.y + 1.0) / 2.0 * height as f32,
                v.z,
            )
        };

        for triangle in triangles {
            let a = screen_transform(triangle.a.position);
            let b = screen_transform(triangle.b.position);
            let c = screen_transform(triangle.c.position);

            let az = a.z;
            let bz = b.z;
            let cz = c.z;

            let a = a.vec2();
            let b = b.vec2();
            let c = c.vec2();

            if (b - a).cross(c - a) > 0.0 {
                if let Some(bb) = BoundRect::from_points(&[a, b, c]).map(|bb| {
                    bb.intersection(screen_bounds)
                })
                {
                    if !bb.is_empty() {
                        let screen_xmin = bb.min.x.floor() as u32;
                        let screen_ymin = bb.min.y.floor() as u32;
                        let screen_xmax = bb.max.x.ceil() as u32;
                        let screen_ymax = bb.max.y.ceil() as u32;

                        let s1 = b - a;
                        let s2 = c - b;
                        let s3 = a - c;

                        for y in screen_ymin..screen_ymax {
                            for x in screen_xmin..screen_xmax {
                                let p = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);
                                let mut apart = s2.cross(p - b) / 2.0 * az;
                                let mut bpart = s3.cross(p - c) / 2.0 * bz;
                                let mut cpart = s1.cross(p - a) / 2.0 * cz;

                                let sum = apart + bpart + cpart;
                                apart /= sum;
                                bpart /= sum;
                                cpart /= sum;

                                if apart >= 0.0 && bpart >= 0.0 && cpart >= 0.0 {
                                    let color = triangle.a.color * apart +
                                        triangle.b.color * bpart +
                                        triangle.c.color * cpart;
                                    self.set_pixel(x, y, vec4_to_color(color));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.framebuffer[(y * self.dimensions.0 + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, c: Color) {
        self.framebuffer[(y * self.dimensions.0 + x) as usize] = c;
    }
}
