use std::sync::Arc;
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder, Section, Text, ab_glyph};
use winit::window::Window;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    glyph_brush: GlyphBrush<()>,
    staging_belt: wgpu::util::StagingBelt,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                trace: wgpu::Trace::Off,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        const FONT: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/mechanical.otf"
        ));
        let font = ab_glyph::FontArc::try_from_slice(FONT).expect("Erreur : police introuvable");

        let glyph_brush = GlyphBrushBuilder::using_font(font).build(&device, surface_format);

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            glyph_brush,
            staging_belt,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(
        &mut self,
        text: &str,
        history: &[String],
        scroll_offset: f32,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        let prompt_y = self.size.height as f32 - 40.0;
        let line_height = 22.0;
        let max_lines = (self.size.height as f32 / line_height) as usize - 2;

        let scroll = scroll_offset
            .min(history.len() as f32 * line_height)
            .max(0.0);

        let visible_history = if history.len() > max_lines {
            &history[history.len() - max_lines..]
        } else {
            history
        };

        let mut y = prompt_y - line_height + scroll;

        for entry in visible_history.iter().rev() {
            self.glyph_brush.queue(Section {
                screen_position: (10.0, y),
                bounds: (self.size.width as f32, self.size.height as f32),
                text: vec![
                    Text::new(entry)
                        .with_color([0.5, 0.8, 1.0, 1.0])
                        .with_scale(30.0),
                ],
                ..Section::default()
            });

            y -= line_height;
        }

        let prompt_text = format!("$ {}", text);

        self.glyph_brush.queue(Section {
            screen_position: (10.0, prompt_y),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![
                Text::new(&prompt_text)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(30.0),
            ],
            ..Section::default()
        });

        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &view,
                self.size.width,
                self.size.height,
            )
            .expect("Erreur lors du dessin du texte");

        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.staging_belt.recall();

        Ok(())
    }
}
