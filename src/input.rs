
//struct
//use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
//use std::collections::HashSet;

pub fn is_pressed(e: &sdl2::EventPump,key:Scancode) -> bool {
    e.keyboard_state().is_scancode_pressed(key)
}