
//struct
//use sdl2::keyboard::Keycode;
use sdl2::{keyboard::Scancode, EventPump};
//use std::collections::HashSet;
use crate::sdl2helper::sdl2objects::Winsdl;

pub fn is_pressed(e: &sdl2::EventPump,key:Scancode) -> bool {
    e.keyboard_state().is_scancode_pressed(key)
}

pub fn mouse_pressed_left(e:&EventPump) -> bool{
    e.mouse_state().left()
}

pub fn mouse_pos(e:&EventPump) -> (i32,i32){
    (e.mouse_state().x(),e.mouse_state().y())
}

pub fn move_mouse_to_center(win: &Winsdl) -> (i32,i32) {
    
    let mouse_util = win.sdl.mouse();
    
    let window_size = win.window.size();
    let window_width = window_size.0;
    let window_height = window_size.1;
    
    let center_x = window_width as i32 / 2;
    let center_y = window_height as i32 / 2;
    
    let change = mouse_pos(&win.event_pump);

    mouse_util.warp_mouse_in_window(&win.window, center_x, center_y);

    let change = (center_x - change.0,center_y - change.1);
    change
}