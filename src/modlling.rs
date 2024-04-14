use sdl2::keyboard::Scancode;
use sdl2::sys::va_list;

use crate::ray_marching_objects::*;
use crate::opengl_shit::*;
use crate::input::*;

pub struct Modlling{
    model_name:String,
    model_code:String,
    line_x:f32,
    line_y:f32,
    dis:f32,
}

impl Modlling{
    pub fn start(s:&mut Scene) -> Modlling{
        //s.clear();
        s.shader_ajjster();
        let cam = &mut s.cam;
        cam.x = 0.0;cam.y = 0.0;cam.z = -10.0;
        cam.angle_x = 0.0;cam.angle_y = 0.0;
        cam.angle_z = 0.0;
        s.add_object("box");
        s.add_object("deimenShit");
        Modlling{
            model_name:String::from("new object"),
            model_code:String::from("retrun 10000"),
            line_x:0.0,
            line_y:0.0,
            dis:10.0,
        }
    }

    pub fn update(&mut self,s:&mut Scene,win:&Winsdl){
        let cam = &mut s.cam;
        if is_pressed(&win.event_pump,Scancode::W) {
            self.line_x += 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::S) {
            self.line_x -= 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::D) {
            self.line_y += 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::A) {
            self.line_y -= 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::E) {
            self.dis += 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::Q) {
            self.dis -= 0.1;
        }

        cam.z = self.line_y.sin() * self.line_x.cos() * -self.dis;
        cam.y = self.line_y.cos() * self.line_x.sin() * -self.dis;
        cam.angle_y = -self.line_x;
        cam.x = self.line_y.sin() * -self.dis;
        cam.z = self.line_x.cos() * self.line_y.cos() * -self.dis;
        cam.angle_x = -self.line_y;


    }
}