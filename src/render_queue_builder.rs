use crate::app_state::AppState;

pub struct RenderQueueBuilder {
}

impl RenderQueueBuilder {
    pub fn build_queue(encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, app_state: &Option<AppState>) {
        match app_state {
            Some(state) => create_queue_for_app_state(encoder, texture_view, state),
            // Default.
            _ => create_green_screen(encoder, texture_view),
        }
    }
}

fn create_queue_for_app_state(encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView, _state: &AppState) {
    // Default
    create_green_screen(encoder, texture_view);
}

// Renders a GREEN screen
fn create_green_screen(encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView,) {
    // Create the renderpass which will clear the screen.
    let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
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