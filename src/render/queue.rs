use std::sync::Arc;
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
            self.add_empty_screen_render_pass(encoder, texture_view, wgpu::Color::WHITE);
            return;
        }

        // Default
        self.add_empty_screen_render_pass(encoder, texture_view, wgpu::Color::BLACK);
    }

    // Renders a screen with specified Color.
    fn add_empty_screen_render_pass(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, color: wgpu::Color) {
        // Create the renderpass which will clear the screen.
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        renderpass.set_pipeline(self.pipelines.get(&PipelineName::Pipeline1).as_ref());
        renderpass.draw(0..3, 0..1);
        // End the renderpass.
        drop(renderpass);
    }
}
