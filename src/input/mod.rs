use legion::Resources;
use raylib::{math::Vector2, *};

use crate::input::event::{InputQueue, InputState};
use raylib::ffi::{KeyboardKey, MouseButton};

pub mod event;
use event::InputEvent;

#[derive(Default)]
pub struct InputReader;

impl InputReader {
    pub fn update(&self, rl: &RaylibHandle, resources: &mut Resources) {
        if let Some(mut state) = resources.get_mut::<InputState>() {
            state.mov_dir = Vector2 { x: 0.0, y: 0.0 };

            if rl.is_key_down(KeyboardKey::KEY_D) {
                state.mov_dir.x += 1.0;
            }

            if rl.is_key_down(KeyboardKey::KEY_W) {
                state.mov_dir.y += -1.0;
            }

            if rl.is_key_down(KeyboardKey::KEY_A) {
                state.mov_dir.x += -1.0;
            }

            if rl.is_key_down(KeyboardKey::KEY_S) {
                state.mov_dir.y += 1.0;
            }

            if state.mov_dir.length() > 1.0 {
                state.mov_dir = state.mov_dir.normalized();
            }

            state.mouse_pos = rl.get_mouse_position();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if let Some(mut queue) = resources.get_mut::<InputQueue>() {
                queue.0.push(InputEvent::Dash);
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            if let Some(mut queue) = resources.get_mut::<InputQueue>() {
                queue.0.push(InputEvent::Shop);
            }
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(mut queue) = resources.get_mut::<InputQueue>() {
                queue.0.push(InputEvent::LeftClick);
            }
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            if let Some(mut queue) = resources.get_mut::<InputQueue>() {
                queue.0.push(InputEvent::RightClick);
            }
        }
    }
}
