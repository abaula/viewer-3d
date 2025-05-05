use std::sync::Arc;
use winit::window::Window;
use crate::model::Model;
use crate::render::queue_source::QueueSource;

pub struct View {
    render_counter: i32,
    queue_source: Box<QueueSource>,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}

impl View {
    pub async fn create(window: Arc<Window>) -> Option<View> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = create_surface(&instance, &window)?;
        let adapter = create_default_adapter(&instance).await?;
        let (device, queue) = request_default_device(&adapter).await?;
        let size = window.inner_size();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];
        let render_counter = 0;
        let queue_source = Box::new(QueueSource::new());

        let view = View {
            render_counter,
            queue_source,
            window,
            device,
            queue,
            size,
            surface,
            surface_format
        };

        // Configure surface for the first time
        view.configure_surface();

        Some(view)
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        // reconfigure the surface
        self.configure_surface();
    }

    pub fn render(&mut self, model: &Option<Model>) {
        self.render_counter += 1;
        println!("render: {}", self.render_counter);
        // Create texture view
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.queue_source.add_to_encoder(&mut encoder, &texture_view, model);
        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }
}

async fn create_default_adapter(instance: &wgpu::Instance) -> Option<wgpu::Adapter> {
    match instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await {
            Ok(adapter) => Some(adapter),
            Err(e) => {
                eprintln!("Ошибка: {}", e);
                None
            },
        }
}

fn create_surface<'a>(instance: &wgpu::Instance, window: &Arc<Window>) -> Option<wgpu::Surface<'a>> {
    match instance.create_surface(window.clone()) {
        Ok(surface) => Some(surface),
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            None
        },
    }
}

async fn request_default_device(adapter: &wgpu::Adapter) -> Option<(wgpu::Device, wgpu::Queue)> {
    match adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await {
            Ok((device, queue)) => Some((device, queue)),
            Err(e) => {
                eprintln!("Ошибка: {}", e);
                None
            },
        }
}