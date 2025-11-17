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

        // 1. Créer l'instance wgpu (point d'entrée)
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // 2. Créer la surface (zone de dessin)
        let surface = instance.create_surface(window).unwrap();

        // 3. Demander un adaptateur (= votre carte graphique)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // 4. Créer le device et la queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        // 5. Configurer la surface
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

        // 7. Créer le GlyphBrush (outil pour dessiner du texte)
        let glyph_brush = GlyphBrushBuilder::using_font(font).build(&device, surface_format);

        // 8. Créer le staging belt (buffer temporaire)
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
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self, text: &str) -> Result<(), wgpu::SurfaceError> {
        // 1. Récupérer la texture actuelle (l'image à afficher)
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // 2. Créer un encodeur de commandes
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // 3. Effacer l'écran en noir
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None, // ← obligatoire depuis wgpu 0.20
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // 4. Préparer le texte à afficher
        let prompt_text = format!("$ {}", text);

        self.glyph_brush.queue(Section {
            screen_position: (10.0, 10.0),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![
                Text::new(&prompt_text)
                    .with_color([0.0, 1.0, 0.0, 1.0]) // Vert
                    .with_scale(40.0),
            ],
            ..Section::default()
        });

        // 5. Dessiner le texte
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

        // 6. Finaliser et envoyer au GPU
        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // 7. Nettoyer pour le prochain frame
        self.staging_belt.recall();

        Ok(())
    }
}
