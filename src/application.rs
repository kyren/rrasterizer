use std::f32;

use vec3::Vector3;
use vec4::Vector4;
use mat4::Matrix4;
use color::Color;
use renderer::{Vertex, Triangle, Renderer};

const TRIANGLES: [Triangle; 12] = [
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: -1.0,
                y: 1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: 1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: -1.0,
                y: 1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: 1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: 1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: -1.0,
                y: 1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: 1.0,
                y: 1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: -1.0,
                y: 1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
    },
    Triangle {
        a: Vertex {
            position: Vector3 {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        },
        b: Vertex {
            position: Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
        c: Vertex {
            position: Vector3 {
                x: 1.0,
                y: -1.0,
                z: -1.0,
            },
            color: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        },
    },
];

pub struct Application {
    renderer: Renderer,
    rotation: f32,
}

impl Application {
    pub fn new(width: u32, height: u32) -> Application {
        Application {
            renderer: Renderer::new(width, height, f32::consts::PI / 3.0),
            rotation: 0.0,
        }
    }

    pub fn render(&mut self) {
        self.rotation += 0.09;
        let transformation = Matrix4::translation(Vector3::new(0.0, 0.0, -5.0)) *
            Matrix4::rotation(Vector3::new(
                self.rotation / 2.0,
                self.rotation,
                self.rotation / 3.0,
            ));

        self.renderer.clear(Color(0, 0, 0, 255));
        self.renderer.render(transformation, &TRIANGLES);
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let Color(r, g, b, _) = self.renderer.get_pixel(x, y);
        (r, g, b)
    }
}
