use config::Config;
use glyph_brush::ab_glyph::FontArc;
use glyph_brush::{Section, Text};
use std::sync::Arc;
use std::sync::Mutex;

mod fps;
mod shared;
pub mod text;

pub struct Style {
    pub screen_position: (f32, f32),
    pub bounds: (f32, f32),
    pub text_scale: f32,
}

pub struct RendererStyles {
    pub scale: f32,
    pub font_size: f32,
    pub term: Style,
    pub tabs_active: Style,
}

impl RendererStyles {
    pub fn new(scale: f32, width: f32, height: f32, font_size: f32) -> RendererStyles {
        Self::mount_styles(scale, width, height, font_size)
    }

    #[inline]
    fn mount_styles(
        width: f32,
        height: f32,
        scale: f32,
        font_size: f32,
    ) -> RendererStyles {
        let yspacing = 30.0;
        RendererStyles {
            scale,
            font_size,
            term: Style {
                screen_position: (10.0 * scale, (yspacing * scale)),
                bounds: (width - ((font_size + 5.0) * scale), height * scale),
                text_scale: font_size * scale,
            },
            tabs_active: Style {
                screen_position: (80.0 * scale, (8.0 * scale)),
                bounds: (width - (40.0 * scale), height * scale),
                text_scale: 15.0 * scale,
            },
        }
    }

    pub fn refresh_styles(&mut self, width: f32, height: f32, scale: f32) {
        *self = Self::mount_styles(width, height, scale, self.font_size);
    }
}

pub struct Renderer {
    pub brush: text::GlyphBrush<()>,
    pub config: Config,
    styles: RendererStyles,
    fps: fps::FramesCounter,
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        config: Config,
        styles: RendererStyles,
    ) -> Result<Renderer, String> {
        match FontArc::try_from_slice(shared::FONT_FIRAMONO) {
            Ok(font_data) => {
                let brush =
                    text::GlyphBrushBuilder::using_font(font_data).build(device, format);
                let fps = fps::FramesCounter::new();
                Ok(Renderer {
                    brush,
                    config,
                    styles,
                    fps,
                })
            }
            Err(err_message) => Err(format!(
                "Renderer error: could not initialize font {err_message:?}"
            )),
        }
    }

    pub fn refresh_styles(&mut self, scale: f32, width: f32, height: f32) {
        self.styles.refresh_styles(width, height, scale);
    }

    pub fn get_current_term_scale(&self) -> f32 {
        self.styles.scale
    }

    pub fn queue(&mut self, arc_m: &Arc<Mutex<String>>, command: String) {
        self.brush.queue(Section {
            screen_position: self.styles.term.screen_position,
            bounds: self.styles.term.bounds,
            text: vec![Text::new(&arc_m.lock().unwrap())
                .with_color(self.config.colors.foreground)
                .with_scale(self.styles.term.text_scale)],
            ..Section::default()
        });

        let fps_text = format!("fps: {:?}", self.fps.tick());
        self.brush.queue(Section {
            screen_position: self.styles.tabs_active.screen_position,
            bounds: self.styles.tabs_active.bounds,
            text: vec![
                Text::new(&command)
                    .with_color(self.config.colors.tabs_active)
                    .with_scale(self.styles.tabs_active.text_scale),
                Text::new(&fps_text)
                    .with_color(self.config.colors.foreground)
                    .with_scale(self.styles.tabs_active.text_scale),
            ],
            layout: glyph_brush::Layout::default_single_line(),
            // ..Section::default() // .line_breaker(glyph_brush::BuiltInLineBreaker::UNi)
            // .v_align(glyph_brush::VerticalAlign::Center)
            // .h_align(glyph_brush::HorizontalAlign::Left)
        });

        // self.brush.queue(Section {
        //     screen_position: ((self.size.width as f32 - 20.0) * scale, (8.0 * scale)),
        //     bounds: (
        //         (self.size.width as f32) - (40.0 * scale),
        //         (self.size.height as f32) * scale,
        //     ),
        //     text: vec![Text::new("■ vim ■ zsh ■ docker")
        //         //(157,165,237)
        //         .with_color([0.89020, 0.54118, 0.33725, 1.0])
        //         .with_scale(14.0 * scale)],
        //     layout: glyph_brush::Layout::default()
        //         // .line_breaker(glyph_brush::BuiltInLineBreaker::UNi)
        //         // .v_align(glyph_brush::VerticalAlign::Center)
        //         .h_align(glyph_brush::HorizontalAlign::Right),
        //     ..Section::default()
        // });
    }
}
