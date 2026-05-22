pub mod commands;
use crate::{helper::PlayerPos, renderer::commands::{DrawCommand, HudCommand}, config::{SCREEN_H, SCREEN_W}};
use legion::Resources;
use raylib::prelude::*;

pub struct RenderQueue(pub Vec<DrawCommand>);
pub struct HudQueue(pub Vec<HudCommand>);

pub struct Renderer {
    pub rl: RaylibHandle,
    thread: RaylibThread,
    pub cam: Camera2D,
    pub(crate) screen_w: i32,
    pub(crate) screen_h: i32,
    font: Font,
}

impl Renderer {
    pub fn new(title: &str, resources: &Resources) -> Renderer {
        let (mut rl, thread) = raylib::init()
            .size(SCREEN_W, SCREEN_H)
            .fullscreen()
            .title(title)
            .build();

        // Chargement de la fonte pixel art
        let font = rl.load_font(&thread, "assets/font/Kenney Pixel.ttf")
            .expect("Impossible de charger la fonte");

        // Filtre point pour rendu pixel art net (pas de flou bilinéaire)
        unsafe {
            raylib::ffi::SetTextureFilter(
                font.texture,
                TextureFilter::TEXTURE_FILTER_POINT as i32,
            );
        }

        let target = resources
            .get::<PlayerPos>()
            .map(|t| Vector2 { x: t.x as f32, y: t.y as f32 })
            .unwrap_or(Vector2::zero());

        let cam = Camera2D {
            offset: Vector2::zero(),
            target,
            rotation: 0.0,
            zoom: 1.0,
        };

        rl.set_target_fps(60);
        Renderer { rl, thread, cam, screen_w: SCREEN_W, screen_h: SCREEN_H, font }
    }

    pub fn render_frame(&mut self, resources: &mut Resources) {
        if let Some(target) = resources.get::<PlayerPos>() {
            self.cam.target = Vector2 { x: target.x as f32, y: target.y as f32 };
            self.cam.offset = Vector2 {
                x: self.screen_w as f32 / 2.0,
                y: self.screen_h as f32 / 2.0,
            };
        }

        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        // --- Rendu monde (coordonnées monde) ---
        {
            let mut d2 = d.begin_mode2D(self.cam);

            let commands = resources
                .get::<RenderQueue>()
                .map(|q| q.0.clone())
                .unwrap_or_default();

            for cmd in &commands {
                match cmd {
                    DrawCommand::Rectangle { x, y, w, h, color } => {
                        d2.draw_rectangle(*x, *y, *w, *h, *color);
                    }
                    DrawCommand::Text { text, x, y, font_size, color } => {
                        d2.draw_text(&text[..], *x, *y, *font_size, *color);
                    }
                    DrawCommand::Circle { x, y, radius, color } => {
                        d2.draw_circle(*x, *y, *radius as f32, *color);
                    }
                }
            }
        }

        // --- Rendu HUD (coordonnées écran, hors mode2D) ---
        let hud_commands = resources
            .get::<HudQueue>()
            .map(|q| q.0.clone())
            .unwrap_or_default();

        for cmd in &hud_commands {
            match cmd {
                HudCommand::Rectangle { x, y, w, h, color } => {
                    d.draw_rectangle(*x, *y, *w, *h, *color);
                }
                HudCommand::Text { text, x, y, font_size, spacing, color } => {
                    d.draw_text_ex(
                        &self.font,
                        &text[..],
                        Vector2 { x: *x as f32, y: *y as f32 },
                        *font_size as f32,
                        *spacing,
                        *color,
                    );
                }
                HudCommand::Circle { x, y, radius, color } => {
                    d.draw_circle(*x, *y, *radius, *color);
                }
            }
        }

        // Vider les queues
        if let Some(mut q) = resources.get_mut::<RenderQueue>() {
            q.0.clear();
        }
        if let Some(mut q) = resources.get_mut::<HudQueue>() {
            q.0.clear();
        }
    }
}