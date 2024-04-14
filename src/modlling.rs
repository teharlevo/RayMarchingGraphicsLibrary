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
}

impl Modlling{
    pub fn start(s:&mut Scene) -> Modlling{
        s.clear();
        s.shader_ajjster();
        let cam = &mut s.cam;
        cam.x = 0.0;cam.y = 0.0;cam.z = -10.0;
        cam.angle_x = 0.0;cam.angle_y = 0.0;
        cam.angle_z = 0.0;
        Modlling{
            model_name:String::from("new object"),
            model_code:String::from("retrun 10000"),
            line_x:0.0,
            line_y:0.0,
        }
    }

    pub fn update(&self,s:&mut Scene,win:&Winsdl){
        if is_pressed(&win.event_pump,Scancode::W) {

        }
    }
}