
use std::time::Instant;

use opengl_shit::Texture;
use sdl2::event::Event;

mod  input;
//use input::*;
mod opengl_shit;
//use opengl_shit::*;
mod shader_maker;
//use shader_maker::*;
mod ray_marching_objects;
use ray_marching_objects::*;
mod modlling;
//use modlling::*;
mod demo_game;
use demo_game::*;

mod sdl2objects;
use sdl2objects::*;
fn main(){
    let win = Winsdl::new(1000,500,"🥳").unwrap();
    create_opengl_contest(1000,500);
    let mut win = win;

    let cam = Camare::new(0.0, 0.0, -3.0);
    let set = SceneSttinges{
        max_rays: 60,
        min_dis_ray: 0.1,
        max_dis_ray: 1000.0,

        color_senstivity:0.1,
        color_offset:10.0,
        colors_rgb: [(0.8, 0.5, 0.4	),(0.2, 0.4, 0.2),(2.0, 1.0, 1.0),	(0.00, 0.25, 0.25),],
    };

    let bchk_grund = Texture::new();
    bchk_grund.load("camera_pitch_yaw_roll.png");
    bchk_grund.bind();
    let mut se = Scene::new(set,cam,bchk_grund,1000,500);
    

    se.add_folder_to_model("src/objects");
    let ob = se.add_object("evil_man");

    ob.z = 3.0;
    ob.angle_x = 3.14/2.0;
    se.set_shader();
    
    let mut time = Instant::now();
    let mut fps = 0;

    //let mut modlling = Modlling::start(&mut se);
    let mut game = DemoGameLogik::new(&win);

    'main: loop {
        for event in win.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}

            }
        }
        //modlling.update(&mut se, &win);
        game.update(&mut se, &win);
        se.draw();
        
        if Instant::now().duration_since(time).as_secs_f32() > 1.0 {
            println!("fps:{}", fps);
            fps = 0;
            time = Instant::now();
        }
        fps = fps + 1;
        win.window.gl_swap_window();
    }
}