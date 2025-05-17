use crate::model::Model;
use crate::render::render_manager::RenderManager;
use std::sync::Arc;
use winit::window::Window;

pub struct View {
    render_counter: i32,
    window: Arc<Window>,
    device: Arc<wgpu::Device>,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    render_manager: RenderManager,
}

impl View {
    pub async fn create(window: Window) -> Option<View> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let window_ptr = Arc::new(window);
        let surface = create_surface(&instance, &window_ptr)?;
        let adapter = create_default_adapter(&instance).await?;
        let (device, queue) = request_default_device(&adapter).await?;
        let size = window_ptr.inner_size();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];
        let render_counter = 0;
        let device_ref = Arc::new(device);
        let render_manager = RenderManager::new(&device_ref);

        let view = View {
            render_counter,
            window: window_ptr,
            device: device_ref,
            queue,
            size,
            surface,
            surface_format,
            render_manager,
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

        // Create surface texture
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        // Create texture view
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let buffers =
            self.render_manager
                .create_command_buffers(&texture_view, &self.surface_format, model);

        match buffers {
            Some(commands) => {
                self.queue.submit(commands);
                self.window.pre_present_notify();
                surface_texture.present();
            }
            _ => {}
        }
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
        .await
    {
        Ok(adapter) => Some(adapter),
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            None
        }
    }
}

fn create_surface<'a>(
    instance: &wgpu::Instance,
    window: &Arc<Window>,
) -> Option<wgpu::Surface<'a>> {
    match instance.create_surface(window.clone()) {
        Ok(surface) => Some(surface),
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            None
        }
    }
}

async fn request_default_device(adapter: &wgpu::Adapter) -> Option<(wgpu::Device, wgpu::Queue)> {
    match adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
    {
        Ok((device, queue)) => Some((device, queue)),
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            None
        }
    }
}
