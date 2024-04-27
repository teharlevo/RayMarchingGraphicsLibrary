use sdl2::keyboard::Scancode;

use crate::{input::{is_pressed, move_mouse_to_center}, Scene, Winsdl};

pub struct DemoGameLogik{
    velosty_y:f32,
}


impl DemoGameLogik{

    pub fn new(win:&Winsdl) -> DemoGameLogik{
        win.sdl.mouse().show_cursor(false);
        _ = move_mouse_to_center(win);
        DemoGameLogik{
            velosty_y:0.0,
        }
    }

    pub fn update(&mut self,s:&mut Scene,win:&Winsdl){
        let speed = if is_pressed(&win.event_pump,Scancode::LShift) {5.0}else{1.0};

        let cam = &mut s.cam;

        self.velosty_y -= 0.1;
        
        cam.y += self.velosty_y;

        if cam.y < 0.0{
            self.velosty_y = 0.0;
            cam.y = 0.0;
        }


        if is_pressed(&win.event_pump,Scancode::W) {
            cam.x += 0.1 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::S) {
            cam.x -= 0.1 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::D) {
            cam.z += 0.1 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::A) {
            cam.z -= 0.1 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::Space) && cam.y == 0.0{
            self.velosty_y = 1.0;
        }

        if !is_pressed(&win.event_pump,Scancode::Escape) {
            let mouse_cange = move_mouse_to_center(win);
            cam.angle_x += mouse_cange.0 as f32/1000.0;
            cam.angle_y -= mouse_cange.1 as f32/1000.0;

            if cam.angle_y >= 3.14/2.1{
                cam.angle_y = 3.14/2.1
            }
            else if cam.angle_y <= -3.14/1.9{
                cam.angle_y = -3.14/1.9
            }
        }
        

    }
}