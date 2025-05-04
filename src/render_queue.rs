use crate::app_state::AppState;

pub struct QueueSource {
}

impl QueueSource {
    pub fn new() -> QueueSource {
        QueueSource {}
    }

    pub fn add_to_encoder(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, app_state: &Option<AppState>) {
        match app_state {
            Some(state) => self.create_queue_for_app_state(encoder, texture_view, state),
            // Default.
            _ => self.create_empty_screen(encoder, texture_view, wgpu::Color::GREEN),
        }
    }

    fn create_queue_for_app_state(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, state: &AppState) {

        if state.visible {
            self.create_empty_screen(encoder, texture_view, wgpu::Color::BLUE);
            return;
        }

        // Default
        self.create_empty_screen(encoder, texture_view, wgpu::Color::BLACK);
    }

    // Renders a screen with specified Color.
    fn create_empty_screen(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, color: wgpu::Color) {
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

        // If you wanted to call any drawing commands, they would go here.

        // End the renderpass.
        drop(renderpass);
    }
}
