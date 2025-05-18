use crate::{
    model::{Model, Vertex},
    render::shader::{ShaderFragmentName, ShaderSource, ShaderVertexName},
};
use std::{cell::RefCell, ops::Range, sync::Arc};
use wgpu::util::DeviceExt;

#[derive(Clone, Debug)]
struct DrawIndexedInfo {
    base_vertex: i32,
    indices_range: Range<u32>,
}

impl DrawIndexedInfo {
    fn new(base_vertex: i32, indices_range: Range<u32>) -> DrawIndexedInfo {
        DrawIndexedInfo {
            base_vertex,
            indices_range,
        }
    }
}

#[derive(Clone, Debug)]
struct RenderData {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instances: Range<u32>,
    draw_info: Vec<DrawIndexedInfo>,
    bg_color: wgpu::Color,
}

impl RenderData {
    fn build(device: &Arc<wgpu::Device>, model: &Model) -> RenderData {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut draw_info = Vec::new();

        let faces = &*model.faces.borrow();
        let mut last_index = 0_u32;
        let mut next_index;
        let mut last_vertex = 0;

        for face in faces {
            vertices.extend(&face.vertices);
            indices.extend(&face.indices);
            next_index = last_index + face.indices.len() as u32;
            let draw_indexed_info = DrawIndexedInfo::new(last_vertex, last_index..next_index);
            draw_info.push(draw_indexed_info);
            last_index = next_index;
            last_vertex += face.vertices.len() as i32;
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let bg_color = RenderData::get_bg_color(model);
        let instances = 0..1;

        RenderData {
            vertex_buffer,
            index_buffer,
            draw_info,
            instances,
            bg_color,
        }
    }

    pub fn destroy(&self) {
        self.index_buffer.destroy();
        self.vertex_buffer.destroy();
    }

    fn get_bg_color(model: &Model) -> wgpu::Color {
        let color = model.bg_color.borrow();
        wgpu::Color {
            r: color.r as f64,
            g: color.g as f64,
            b: color.b as f64,
            a: color.a as f64,
        }
    }
}

pub struct RenderManager {
    device: Arc<wgpu::Device>,
    shaders: ShaderSource,
    render_data: RefCell<Option<RenderData>>,
    depth_texture_view: RefCell<Option<wgpu::TextureView>>,
}

impl RenderManager {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.

    pub fn new(device: &Arc<wgpu::Device>) -> RenderManager {
        RenderManager {
            device: Arc::clone(device),
            shaders: ShaderSource::new(device),
            render_data: RefCell::new(None),
            depth_texture_view: RefCell::new(None),
        }
    }

    pub fn destroy(&self) {
        let render_data_ref = self.render_data.borrow();
        let render_data = render_data_ref.as_ref().unwrap();
        render_data.destroy();
    }

    pub fn create_depth_texture(
        &self,
        surface_config: &RefCell<Option<wgpu::SurfaceConfiguration>>,
    ) {
        let surface_config_ref = surface_config.borrow();
        let config = surface_config_ref.as_ref().unwrap();

        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = self.device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.depth_texture_view.replace(Some(view));
    }

    pub fn create_command_buffers(
        &self,
        texture_view: &wgpu::TextureView,
        texture_format: &wgpu::TextureFormat,
        model: &Option<Model>,
    ) -> Option<Vec<wgpu::CommandBuffer>> {
        match model {
            Some(state) if state.visible => {
                self.create_drawing_commands(&texture_view, &texture_format, state)
            }
            _ => self.create_empty_screen_commands(&texture_view, wgpu::Color::BLACK),
        }
    }

    fn create_empty_screen_commands(
        &self,
        texture_view: &wgpu::TextureView,
        color: wgpu::Color,
    ) -> Option<Vec<wgpu::CommandBuffer>> {
        let mut encoder = self.device.create_command_encoder(&Default::default());

        // Create the renderpass which will clear the screen.
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // End the renderpass.
        drop(renderpass);

        let mut commands = Vec::new();
        commands.push(encoder.finish());

        Some(commands)
    }

    fn create_drawing_commands(
        &self,
        texture_view: &wgpu::TextureView,
        texture_format: &wgpu::TextureFormat,
        model: &Model,
    ) -> Option<Vec<wgpu::CommandBuffer>> {
        let mut encoder = self.device.create_command_encoder(&Default::default());

        let render_data_ref = self.get_render_data(model).borrow();
        let render_data = render_data_ref.as_ref().unwrap();

        let depth_texture_view_ref = self.depth_texture_view.borrow();
        let depth_texture_view = depth_texture_view_ref.as_ref().unwrap();

        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // don't clear texture_view, use it as is.
                    //load: wgpu::LoadOp::Load,
                    // ...or clear it with the color
                    load: wgpu::LoadOp::Clear(render_data.bg_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let pipline = self.get_pipline(texture_format);
        renderpass.set_pipeline(&pipline);

        renderpass.set_vertex_buffer(0, render_data.vertex_buffer.slice(..));
        renderpass.set_index_buffer(
            render_data.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        for draw_info in &render_data.draw_info {
            renderpass.draw_indexed(
                draw_info.indices_range.clone(),
                draw_info.base_vertex.clone(),
                render_data.instances.clone(),
            );
        }

        // End the renderpass.
        drop(renderpass);

        let mut commands = Vec::new();
        commands.push(encoder.finish());

        Some(commands)
    }

    fn get_render_data(&self, model: &Model) -> &RefCell<Option<RenderData>> {
        let mut value = self.render_data.borrow_mut();

        value.get_or_insert_with(|| RenderData::build(&self.device, model));

        &self.render_data
    }

    fn get_pipline(&self, texture_format: &wgpu::TextureFormat) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("model render pipline"),
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
                        format: texture_format.clone(),
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
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: Self::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less, // 1.
                    stencil: wgpu::StencilState::default(),     // 2.
                    bias: wgpu::DepthBiasState::default(),
                }),
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

        pipeline
    }
}
