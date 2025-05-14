use std::{cell::RefCell, collections::HashMap, sync::Arc};
use crate::{model::Vertex, render::shader::{ShaderFragmentName, ShaderSource, ShaderVertexName}};

#[derive(strum_macros::Display, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PipelineName {
    Pipeline1,
}

pub struct PipelineSource {
    pub device: Arc<wgpu::Device>,
    pub texture_format: wgpu::TextureFormat,
    pub shaders: ShaderSource,
    pipelines: RefCell<HashMap<PipelineName, Arc<wgpu::RenderPipeline>>>,
}

impl PipelineSource {
    pub fn new(device: &Arc<wgpu::Device>, texture_format: &wgpu::TextureFormat) -> PipelineSource {
        let shaders = ShaderSource::new(device);

        PipelineSource {
            device: Arc::clone(device),
            texture_format: texture_format.clone(),
            shaders,
            pipelines: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &PipelineName) -> Arc<wgpu::RenderPipeline> {

        if ! self.pipelines.borrow().contains_key(key) {
            self.pipelines.borrow_mut().insert(key.clone(), self.create_pipeline(key));
        }

        Arc::clone(&self.pipelines.borrow()[key])
    }

    fn create_pipeline(&self, key: &PipelineName) -> Arc<wgpu::RenderPipeline> {
        match key {
            PipelineName::Pipeline1 => self.create_pipeline1(),
        }
    }

    fn create_pipeline1(&self) -> Arc<wgpu::RenderPipeline> {
        let render_pipeline_layout =
            self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let pipeline =
            self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(PipelineName::Pipeline1.to_string().as_str()),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &self.shaders.vertex_shaders[&ShaderVertexName::Vertex1],
                    entry_point: Some("main"),
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.shaders.fragment_shaders[&ShaderFragmentName::Fragment1],
                    entry_point: Some("main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.texture_format.clone(),
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    //cull_mode: Some(wgpu::Face::Back),
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

        Arc::new(pipeline)
    }
}