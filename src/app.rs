use std::sync::Arc;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};
use crate::draw_state::DrawState;
use crate::app_state::AppState;

pub struct App {
    draw_state: Option<DrawState>,
    app_state: Option<AppState>,
}

impl App {
    pub fn new() -> App {
        App {
            draw_state: None,
            app_state: Some(AppState::new()),
        }
    }

    pub fn set_visible(&mut self, visible: bool) {

        match self.app_state.as_mut() {
            Some(app_state) => {

                if app_state.visible == visible {
                    return;
                }

                // Set new visible value.
                app_state.visible = visible;
                // Redraw.
                self.request_redraw_window();
            },
            _ => {}
        }
    }

    fn request_redraw_window(&self) {
        match self.draw_state.as_ref() {
            Some(draw_state) => {
                draw_state.request_redraw();
            },
            _ => {}
        }
    }

    fn render(&mut self) {
        match self.draw_state.as_mut() {
            Some(draw_state) => {
                draw_state.render(&self.app_state)
            },
            _ => {}
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        match self.draw_state.as_mut() {
            Some(draw_state) => {
                draw_state.resize(size)
            },
            _ => {}
        }
    }
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

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                self.resize(size);
            }
            _ => (),
        }
    }
}