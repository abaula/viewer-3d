use std::cell::RefCell;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: Color,
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Clone, Debug)]
pub struct Face {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

pub struct Model {
    pub visible: bool,
    pub bg_color: RefCell<Color>,
    pub faces: RefCell<Vec<Face>>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            visible: false,
            bg_color: RefCell::new(Model::create_color([0.1, 0.2, 0.3])),
            faces: RefCell::new(Model::create_faces()),
        }
    }

    fn create_faces() -> Vec<Face> {
        [
            Face {
                vertices: Model::create_vertices1(),
                indices: Model::create_indices1(),
            },
            Face {
                vertices: Model::create_vertices2(),
                indices: Model::create_indices2(),
            },
        ]
        .to_vec()
    }

    fn create_vertices1() -> Vec<Vertex> {
        [
            Vertex {
                position: [-0.0868241, 0.49240386, 0.3],
                color: Model::create_color([0.15, 0.0, 0.5]),
            }, // A
            Vertex {
                position: [-0.49513406, 0.06958647, 0.3],
                color: Model::create_color([0.5, 0.0, 0.15]),
            }, // B
            Vertex {
                position: [-0.21918549, -0.44939706, 0.3],
                color: Model::create_color([0.15, 0.10, 0.1]),
            }, // C
            Vertex {
                position: [0.35966998, -0.3473291, 0.1],
                color: Model::create_color([0.2, 0.5, 0.5]),
            }, // D
            Vertex {
                position: [0.44147372, 0.2347359, 0.1],
                color: Model::create_color([0.5, 0.2, 0.1]),
            }, // E
        ]
        .to_vec()
    }

    fn create_indices1() -> Vec<u16> {
        [0, 1, 4, 1, 2, 4, 2, 3, 4].to_vec()
    }

    fn create_vertices2() -> Vec<Vertex> {
        [
            Vertex {
                position: [-0.2868241, 0.49240386, 0.2],
                color: Model::create_color([0.1, 0.1, 0.5]),
            }, // A
            Vertex {
                position: [-0.69513406, 0.14958647, 0.2],
                color: Model::create_color([0.1, 0.2, 0.15]),
            }, // B
            Vertex {
                position: [-0.41918549, -0.44939706, 0.2],
                color: Model::create_color([0.5, 0.3, 0.1]),
            }, // C
            Vertex {
                position: [0.15966998, -0.3473291, 0.2],
                color: Model::create_color([0.9, 0.2, 0.5]),
            }, // D
            Vertex {
                position: [0.24147372, 0.2347359, 0.2],
                color: Model::create_color([0.9, 0.1, 0.9]),
            }, // E
        ]
        .to_vec()
    }

    fn create_indices2() -> Vec<u16> {
        [0, 1, 4, 1, 2, 4, 2, 3, 4].to_vec()
    }

    fn create_color(values: [f32; 3]) -> Color {
        Color {
            r: values[0],
            g: values[1],
            b: values[2],
            a: 1.0,
        }
    }
}
