use std::collections::HashMap;

#[derive(strum_macros::Display, Debug, PartialEq, Eq, Hash)]
pub enum ShaderVertexName {
    Vertex1,
}

#[derive(strum_macros::Display, Debug, PartialEq, Eq, Hash)]
pub enum ShaderFragmentName {
    Fragment1,
}

pub struct ShaderSource {
    pub vertex_shaders: HashMap<ShaderVertexName, wgpu::ShaderModule>,
    pub fragment_shaders: HashMap<ShaderFragmentName, wgpu::ShaderModule>,
}

impl ShaderSource {
    pub fn new(device: &wgpu::Device) -> ShaderSource {
        let mut vertex_shaders = HashMap::new();
        let mut fragment_shaders = HashMap::new();

        // Vertex1
        let vertex1_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(ShaderVertexName::Vertex1.to_string().as_str()),
            source: wgpu::ShaderSource::Wgsl(include_str!("resources/shaders/vertex/vertex1.wgsl").into()),
        });
        
        vertex_shaders.insert(ShaderVertexName::Vertex1, vertex1_module);

        // Fragment1
        let fragment1_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(ShaderFragmentName::Fragment1.to_string().as_str()),
            source: wgpu::ShaderSource::Wgsl(include_str!("resources/shaders/fragment/fragment1.wgsl").into()),
        });
        
        fragment_shaders.insert(ShaderFragmentName::Fragment1, fragment1_module);

        ShaderSource {
            vertex_shaders,
            fragment_shaders,
        }
    }
}