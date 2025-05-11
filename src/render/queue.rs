use std::sync::Arc;
use wgpu::util::DeviceExt;

use crate::model::Model;
use crate::render::pipeline::{PipelineSource, PipelineName};

pub struct QueueSource {
    pipelines: Box<PipelineSource>,
}

impl QueueSource {
    pub fn new(device: &Arc<wgpu::Device>, texture_format: &wgpu::TextureFormat) -> QueueSource {
        let pipelines = Box::new(PipelineSource::new(device, texture_format));

        QueueSource {
            pipelines,
        }
    }

    pub fn add_command_queue(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, model: &Option<Model>) {
        match model {
            Some(state) => self.add_queue_for_app_state(encoder, texture_view, state),
            // Default.
            _ => self.add_empty_screen_render_pass(encoder, texture_view, wgpu::Color::GREEN),
        }
    }

    fn add_queue_for_app_state(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, model: &Model) {

        if model.visible {
            self.add_model_render_pass(encoder, texture_view, model);
            return;
        }

        // Default
        self.add_empty_screen_render_pass(encoder, texture_view, wgpu::Color::BLACK);
    }

    // Renders a screen with specified Color.
    fn add_empty_screen_render_pass(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, color: wgpu::Color) {
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
    }

    fn add_model_render_pass(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, model: &Model) {

        let device = &*self.pipelines.device;
        let vertices = &*model.vertices.borrow();
        let indices = &*model.indices.borrow();

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;


        // Create the renderpass which will clear the screen.
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(QueueSource::get_bg_color(model)),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        renderpass.set_pipeline(self.pipelines.get(&PipelineName::Pipeline1).as_ref());

        renderpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        renderpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        renderpass.draw_indexed(0..num_indices, 0, 0..1);

        // End the renderpass.
        drop(renderpass);
    }

    fn get_bg_color(model: &Model) -> wgpu::Color {
        let bg_color = model.bg_color.borrow();
        wgpu::Color {
            r: bg_color.r as f64,
            g: bg_color.g as f64,
            b: bg_color.b as f64,
            a: bg_color.a as f64,
        }
    }
}
