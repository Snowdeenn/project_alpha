pub mod commands;
use crate::renderer::commands::DrawCommand;
use legion::{Resources};
use raylib::prelude::*;

pub struct RenderQueue(pub Vec<DrawCommand>);

pub struct Renderer {
    pub rl: RaylibHandle,
    thread: RaylibThread,
}

impl Renderer {
    pub fn new(w: i32, h: i32, title: &str) -> Renderer{
       let (mut rl, thread) = raylib::init().size(w, h).title(title).build();
       rl.set_target_fps(60);
       Renderer { rl, thread }
    }

    pub fn render_frame(&mut self, resources: &mut Resources) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        let commands = resources
            .get::<RenderQueue>()
            .map(|q| q.0.clone())
            .unwrap_or_default();

        for cmd in &commands {
            match cmd {
                DrawCommand::Rectangle { x, y, w, h, color } => {
                    d.draw_rectangle(*x, *y, *w, *h, *color);
                }
            }
        }

        if let Some(mut q) = resources.get_mut::<RenderQueue>() {
            q.0.clear();
        }
    }
}
