pub mod commands;

use crate::renderer::commands::DrawCommand;
use legion::Resources;
use raylib::prelude::*;

pub struct RenderQueue(pub Vec<DrawCommand>);

pub struct Renderer {
    pub rl: RaylibHandle,
    thread: RaylibThread,
    pub cam: Camera2D,
    pub(crate) screen_w: i32,
    pub(crate) screen_h: i32,
}

const SCREEN_H: i32 = 1080;
const SCREEN_W: i32 = 1920;

impl Renderer {
    pub fn new(title: &str, resources: &Resources) -> Renderer {
        let (mut rl, thread) = raylib::init()
            .size(SCREEN_W, SCREEN_H)
            .fullscreen()
            .title(title)
            .build();

        let target = resources
            .get::<CameraTarget>()
            .map(|t| t.pos.clone())
            .unwrap_or_default();

        let cam = Camera2D {
            offset: Vector2::zero(),
            target,
            rotation: 0.0,
            zoom: 1.0,
        };

        rl.set_target_fps(60);
        Renderer { rl, thread, cam, screen_w: SCREEN_W, screen_h: SCREEN_H }
    }

    pub fn render_frame(&mut self, resources: &mut Resources) {
        if let Some(target) = resources.get::<CameraTarget>() {
            self.cam.target = Vector2 {
                x: target.pos.x,
                y: target.pos.y,
            };
            self.cam.offset = Vector2 {
                x: self.screen_w as f32/ 2.0,
                y: self.screen_h as f32 / 2.0,
            };
        }

        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

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
                }
            }
        }

        if let Some(mut q) = resources.get_mut::<RenderQueue>() {
            q.0.clear();
        }
    }
}

pub struct CameraTarget {
    pub pos: Vector2,
}
