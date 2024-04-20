
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

mod  input;
use input::*;
mod opengl_shit;
//use opengl_shit::*;
mod shader_maker;
//use shader_maker::*;
mod ray_marching_objects;
use ray_marching_objects::*;
mod modlling;
use modlling::*;
fn main(){
    let win = create_window(1000,500,"🥳");
    let mut win = win;

    let cam = Camare::new(0.0, 0.0, -3.0);
    let set = SceneSttinges{
        max_rays: 7,
        min_dis_ray: 0.1,
        max_dis_ray: 1000.0,

        color_senstivity:0.005,
        color_offset:10.0,
        colors_rgb: [(0.8, 0.5, 0.4	),(0.2, 0.4, 0.2),(2.0, 1.0, 1.0),	(0.00, 0.25, 0.25),],
    };
    let mut se = Scene::new(set,cam,1000,500);
    se.set_shader(); 
    let k = 4.0;
    for i in 0..30{
        let box1 = se.add_object("trus") ;
        box1.z = i as f32 * k - 30.0 * k;
        box1.angle_x = 3.14/2.0;box1.scale = 1.3;
    }
    let mut time = Instant::now();
    let mut fps = 0;

    let mut w = Modlling::start(&mut se);

    'main: loop {
        for event in win.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}

            }
        }
        w.update(&mut se, &win);
        se.update();
        draw(&win);
        if Instant::now().duration_since(time).as_secs_f32() > 1.0 {
            println!("fps:{}", fps);
            fps = 0;
            time = Instant::now();
        }
        fps = fps + 1;
    }
}