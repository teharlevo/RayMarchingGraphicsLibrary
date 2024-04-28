
use std::time::Instant;
use input::mouse_pressed_left;
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
use modlling::*;
mod demo_game;
use demo_game::*;

mod sdl2objects;
use sdl2objects::*;
fn main(){
    let win = Winsdl::new(1000,500,"🥳").unwrap();
    create_opengl_contest(1000,500);
    let mut win = win;

    let cam = Camare::new(0.0, 0.0, -3.0);
    let bchk_grund = Texture::new();
    _ = bchk_grund.load("camera_pitch_yaw_roll.png");

    let set = SceneSttinges{
        max_rays: 60,
        min_dis_ray: 0.1,
        max_dis_ray: 1000.0,

        color_senstivity:0.1,
        color_offset:10.0,
        colors_rgb: [(0.8, 0.5, 0.4	),(0.2, 0.4, 0.2),(2.0, 1.0, 1.0),	(0.00, 0.25, 0.25),],
        background:SceneBackGround::Color(0.3, 0.1, 0.1),
    };

    let mut game_mode = 0;
    
    let mut se = Scene::new(set,cam,1000,500);
    

    se.add_folder_to_model("src/objects");
    
    se.update_shader();
    
    let mut time = Instant::now();
    let mut fps = 0;

    let mut modlling:Modlling = Modlling::empty();
    let mut game:DemoGameLogik = DemoGameLogik::empty();

    'main: loop {
        for event in win.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}

            }
        }

        (game_mode,game,modlling) = menu_update(game_mode,&mut se, &win,game,modlling);
        if game_mode == 1{
            modlling.update(&mut se, &win);
        }
        else if game_mode == 2 {
            game.update(&mut se, &win);   
        }
        //se.draw();
        
        if Instant::now().duration_since(time).as_secs_f32() > 1.0 {
            println!("fps:{}", fps);
            fps = 0;
            time = Instant::now();
        }
        fps = fps + 1;
        win.window.gl_swap_window();
    }
}

fn menu_update(gm:i32,s:&mut Scene,win:&Winsdl,mut game:DemoGameLogik,mut modling:Modlling) -> (i32,DemoGameLogik,Modlling){
    let mut new_gm = gm;
    if gm == 0{
        if mouse_pressed_left(&win.event_pump){
            game = DemoGameLogik::new(s,win);
            new_gm = 2;
        }else if mouse_pressed_left(&win.event_pump){
            modling = Modlling::start(s, win);
            new_gm = 1;
        }
    }
    (new_gm,game,modling)
}