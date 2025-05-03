use std::sync::Arc;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};
use crate::draw_state::DrawState;
use crate::app_state::AppState;

#[derive(Default)]
pub struct App {
    draw_state: Option<DrawState>,
    app_state: Option<AppState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let draw_state = pollster::block_on(DrawState::new(window.clone()));
        self.draw_state = Some(draw_state);

        let app_state = pollster::block_on(AppState::new());
        self.app_state = Some(app_state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let draw_state = self.draw_state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                draw_state.render(&self.app_state);
                // Emits a new redraw requested event.
                //draw_state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                draw_state.resize(size);
            }
            _ => (),
        }
    }
}