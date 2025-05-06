use std::collections::HashMap;
use crate::render::shader::{ShaderSource, ShaderVertexName, ShaderFragmentName};

#[derive(strum_macros::Display, Debug, PartialEq, Eq, Hash)]
pub enum PipelineName {
    Pipline1,
}

pub struct PipelineSource {
    pub pipelines: HashMap<PipelineName, wgpu::RenderPipeline>,
}

impl PipelineSource {
    pub fn new(device: &wgpu::Device, texture_format: &wgpu::TextureFormat, shader_source: &ShaderSource) -> PipelineSource {
        let mut pipelines = HashMap::new();

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_source.vertex_shaders[&ShaderVertexName::Vertex1],
                    entry_point: Some("main"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_source.fragment_shaders[&ShaderFragmentName::Fragment1],
                    entry_point: Some("main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: texture_format.clone(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                // If the pipeline will be used with a multiview render pass, this
                // indicates how many array layers the attachments will have.
                multiview: None,
                cache: None,
            });

        pipelines.insert(PipelineName::Pipline1, pipeline);

        PipelineSource {
            pipelines
        }
    }
}