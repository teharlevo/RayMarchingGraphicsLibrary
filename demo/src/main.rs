use std::time::Instant;
use input::{mouse_pos, mouse_pressed_left};
use rmgl::{opengl_objects, ray_marching_objects};
use sdl2::event::Event;

pub mod input;
pub mod modlling;
pub mod demo_game;
pub mod sdl2objects;

//use input::*;
use opengl_objects::*;

//use shader_maker::*;

use ray_marching_objects::*;
use modlling::*;
use demo_game::*;
use sdl2objects::*;

fn main(){
    let win = Winsdl::new(1000,500,"🥳").unwrap();
    create_opengl_contest(1000,500);
    crate_world_window();
    let mut win = win;

    let cam = Camare::new(0.0, 0.0, 0.0);

    let set = SceneSttinges{
        max_rays: 2000,
        min_dis_ray: 0.1,
        max_dis_ray: 1500.0,
        show_above_min_dis_errors:false,

        color_senstivity:0.03,
        color_offset:0.0,
        colors_rgb: [(0.8, 0.5, 0.4	),(0.2, 0.4, 0.2),(2.0, 1.0, 1.0),	(0.00, 0.25, 0.25),],
        background:SceneBackGround::ContinuationOfRay(0.000015, 0.0),
        dis_from_zero: false,
    };
    
    let mut se = Scene::new(set.clone(),cam,1000,500);
    
    let mut time = Instant::now();
    let mut lest_frame = Instant::now();
    let mut fps = 0;
    let mut dt = 0.0;

    let mut mode = menu_start(&mut se, &win, set.clone());

    'main: loop {
        for event in win.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}

            }
        }

        match &mut mode {
            Mode::Menu => {
                mode = menu_update(&mut se, &win,mode,dt);
                se.draw();
            },
            Mode::DemoGame(dg) => {
                if dg.update(&mut se, &win,dt){
                    mode = menu_start(&mut se,&win,set.clone());
                }
            },
            Mode::Modling(m) => {
                if m.update(&mut se, &win,dt){
                    mode = menu_start(&mut se,&win,set.clone());
                }
            },
        }
        
        if Instant::now().duration_since(time).as_secs_f32() > 1.0 {
            println!("fps:{}", fps);
            fps = 0;
            time = Instant::now();
        }
        fps = fps + 1;
     
        dt = Instant::now().duration_since(lest_frame).as_secs_f32();
        lest_frame = Instant::now();
    
        win.window.gl_swap_window();
    }
}

fn menu_update(s:&mut Scene,win:&Winsdl,mode:Mode,dt:f32) -> Mode{
    match mode{
        Mode::Menu => {
            let mp = mouse_pos(&win.event_pump);
            if mouse_pressed_left(&win.event_pump) && mp.0 > 0 && mp.0 < 500
            && mp.1 > 0 && mp.1 < 500{
                return Mode::DemoGame(DemoGameLogik::new(s,win));
            }else if mouse_pressed_left(&win.event_pump) && mp.0 > 500 && mp.0 < 1000
            && mp.1 > 0 && mp.1 < 500 {
                return Mode::Modling(Modlling::start(s, win));
            }
            s.sttinges.max_dis_ray += 600.0 * dt;
        },
        Mode::DemoGame(_) => {},
        Mode::Modling(_) => {},
    }
    return Mode::Menu;
}

fn menu_start(s:&mut Scene,win:&Winsdl,sttinges :SceneSttinges) -> Mode{
    s.sttinges = sttinges;
    s.cam = Camare::new(0.0,0.0,0.0);
    win.sdl.mouse().show_cursor(true);
    s.add_folder_to_model("demo/objects");
    
    s.update_shader();
    let g = s.add_object("demo_word");
    g.z = -2.0;
    let g = s.add_object("mode_word");
    g.z = -2.0;
    let g = s.add_object("lin_word");
    g.z = -2.0;
    g.x = 3.0;
    let g = s.add_object("G");
    //g.z = -2.0;
    g.scale = 0.8;

    return Mode::Menu;
}

enum Mode {
    Menu,
    DemoGame(DemoGameLogik),
    Modling(Modlling),

}