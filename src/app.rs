use std::sync::Arc;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};
use crate::view::View;
use crate::model::Model;

pub struct App {
    view: Option<View>,
    model: Option<Model>,
}

impl App {
    pub fn new() -> App {
        App {
            view: None,
            model: Some(Model::new()),
        }
    }

    pub fn set_visible(&mut self, visible: bool) {

        match self.model.as_mut() {
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
        match self.view.as_ref() {
            Some(draw_state) => {
                draw_state.request_redraw();
            },
            _ => {}
        }
    }

    fn render(&mut self) {
        match self.view.as_mut() {
            Some(draw_state) => {
                draw_state.render(&self.model)
            },
            _ => {}
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        match self.view.as_mut() {
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

        let draw_state = pollster::block_on(View::new(window.clone()));
        self.view = Some(draw_state);

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