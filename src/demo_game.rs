use sdl2::keyboard::Scancode;
use rand::{Rng, RngCore};

use crate::{input::*, opengl_shit::FrameBuffer, Camare, Scene, SceneBackGround, Winsdl};

const BACKGRUND_SLOOWNES:f32 = 5.0;
const SENSTIVITY:f32 = 0.08;
const PLAYER_SPEED:f32 = 30.0;
const VELOSTY_LOSE_PER_SEC:f32 = 3.0;
const BALL_VELOSTY_LOSE_PER_SEC:f32 = 22.0;
const JUMP_FORCE:f32 = 1.5;

const BALL_POAINTER:usize = 2;
const RADIOS_OF_BALL:f32 = 3.5;
pub struct DemoGameLogik{
    //hund_secne:Scene,
    background_secne:Scene,
    velosty_y:f32,
    cam_look_x:f32,
    cam_look_y:f32,
    ball:Ball,
    shot:Option<Shot>,
    game_mode:GameMode,
}

enum GameMode {
    Play,
    Lose,
}


impl DemoGameLogik{

    pub fn new(s:&mut Scene,win:&Winsdl) -> DemoGameLogik{

        s.clear_objects();
        win.sdl.mouse().show_cursor(false);
        _ = move_mouse_to_center(win);

        let (bs,ball) = DemoGameLogik::crate_game_world(s,win);
        
        DemoGameLogik{
            //background_framebuffer:fb,
            //hund_secne:hs, 
            velosty_y:0.0,
            background_secne: bs,
            cam_look_x:0.0,
            cam_look_y:0.0,
            ball:ball,
            shot:None,
            game_mode: GameMode::Play,
        }
    }

    fn crate_game_world(s:&mut Scene,win:&Winsdl) -> (Scene,Ball){
        let mut bs = Scene::new(s.sttinges.clone(), s.cam.clone(), s.get_scene_width(),s.get_scene_height());
        bs.add_folder_to_model("objects");
        bs.update_shader();
        bs.sttinges.background = SceneBackGround::Color(1.0, 1.0, 1.0);
        bs.sttinges.color_senstivity = 0.003;
        bs.sttinges.max_dis_ray = 1000.0;
        bs.sttinges.show_above_min_dis_errors = true;
        bs.sttinges.max_rays = 60;

        let ob = s.add_object("floor");
        ob.y = -2.0;

        
        let ob = s.add_object("golef_cors");
        ob.scale = 1.5;
        ob.angle_y = -3.14/2.0;

        
        
        let ob = bs.add_object("death");
        ob.angle_z = 3.14/2.0;

        let fb = FrameBuffer::new(s.get_scene_width(),s.get_scene_height());
        s.sttinges.background = SceneBackGround::FrameBuffer(fb);
        let ob = s.add_object("sphere");
        ob.scale = RADIOS_OF_BALL;

        s.cam.z = -19.5;

        let ball = Ball{
            velosty: (0.0,0.0,0.0),
            x: 0.0,
            y: 30.0,
            z: -6.0,
            r:RADIOS_OF_BALL,
        };
        (bs,ball)
    }

    pub fn update(&mut self,s:&mut Scene,win:&Winsdl,dt:f32) -> bool{
        self.cam_controall_and_col(s, win, dt);

        match self.game_mode {
            GameMode::Play => {
                self.cam_cal(s);
                match &mut s.sttinges.background {
                    SceneBackGround::FrameBuffer(fb) => {
                        self.upade_backgrund(&s.cam,fb)
                    },
                    SceneBackGround::Image(_) => {},
                    SceneBackGround::Color(_, _, _) => {},
                    SceneBackGround::ContinuationOfRay(_, _) => {},
                }
                if self.ball_upadte(win,s, dt){
                    s.clear_objects();
                    s.sttinges.background = SceneBackGround::ContinuationOfRay(0.00003,1.0);
                    let ob = s.add_object("p");
                    ob.x = 90.0;
                    ob.scale = 2.0;
                    ob.angle_z = 3.14;
                    ob.angle_x = 3.14;
                    ob.y = 50.0;

                    let ob = s.add_object("r");
                    ob.x = 50.0;
                    ob.scale = 2.0;
                    ob.angle_y = 3.14;
                    ob.y = 50.0;

                    let ob = s.add_object("e");
                    ob.scale = 1.3;
                    ob.angle_y = 3.14;
                    ob.x = 20.0;
                    ob.y = 40.0;

                    let ob = s.add_object("s");
                    ob.x = -15.0;
                    ob.y = 40.0;
                    ob.scale = 1.5;
                    ob.angle_y = 3.14;

                    let ob = s.add_object("s");
                    ob.angle_y = 3.14;
                    ob.x = 5.0;
                    ob.scale = 1.5;
                    ob.x = -40.0;
                    ob.y = 40.0;

                    let ob = s.add_object("s");
                    ob.angle_y = 3.14;
                    ob.x = 90.0;
                    ob.scale = 1.5;

                    let ob = s.add_object("p");
                    ob.x = 60.0;
                    ob.scale = 2.0;
                    ob.angle_z = 3.14;
                    ob.angle_x = 3.14;

                    let ob = s.add_object("a");
                    ob.x = 30.0;
                    ob.y = 20.0;
                    ob.scale = 2.0;
                    ob.angle_z = 3.14;
                    ob.angle_x = 3.14;

                    let ob = s.add_object("c");
                    ob.x = 0.0;
                    ob.scale = 2.0;
                    ob.angle_z = -3.14/2.0;
                    ob.angle_x = 3.14;

                    let ob = s.add_object("e");
                    ob.scale = 1.3;
                    ob.angle_y = 3.14;
                    ob.x = -20.0;

                    s.sttinges.color_offset = 0.0;
                    s.sttinges.color_senstivity = s.sttinges.color_senstivity * 0.25;
                    self.game_mode = GameMode::Lose;
                }
                
            },
            GameMode::Lose => {
                if self.update_loss(win,s){
                    self.game_mode = GameMode::Play;  
                }
            },
        }

        s.draw();
            
        if is_pressed(&win.event_pump,Scancode::Escape){
            s.clear();
            return true;
        }   

        false
    }

    fn cam_controall_and_col(&mut self,s:&mut Scene,win:&Winsdl,dt:f32){
        
        let cam = &mut s.cam;

        self.velosty_y -= VELOSTY_LOSE_PER_SEC * dt;
        
        cam.y += self.velosty_y;

        if cam.y < 0.0{
            self.velosty_y = 0.0;
            cam.y = 0.0;
        }


        let mut move_f = 0.0;
        let mut move_r = 0.0;

        if is_pressed(&win.event_pump,Scancode::W) {
            move_f -= PLAYER_SPEED * dt;
        }
        if is_pressed(&win.event_pump,Scancode::S) {
            move_f += PLAYER_SPEED * dt;
        }
        if is_pressed(&win.event_pump,Scancode::D) {
            move_r -= PLAYER_SPEED * dt;
        }
        if is_pressed(&win.event_pump,Scancode::A) {
            move_r += PLAYER_SPEED * dt;
        }
        if is_pressed(&win.event_pump,Scancode::Space) && cam.y == 0.0{
            self.velosty_y = JUMP_FORCE;
        }
        let norlizer = (cam.dir.0 * cam.dir.0 + cam.dir.2 * cam.dir.2).sqrt();

        cam.x += move_f * cam.dir.0/norlizer;
        cam.z += move_f * cam.dir.2/norlizer;

        cam.x -= move_r * cam.dir.2/norlizer;
        cam.z += move_r * cam.dir.0/norlizer;

        let mouse_cange = move_mouse_to_center(win);
        self.cam_look_x -= mouse_cange.0 as f32 * dt * SENSTIVITY;
        self.cam_look_y -= mouse_cange.1 as f32 * dt * SENSTIVITY;
        
        if self.cam_look_y >= 3.14/2.0{
            self.cam_look_y = 3.14/2.0;
        }
        else if self.cam_look_y <= -3.14/2.0{
            self.cam_look_y = -3.14/2.0;
        }
        //println!("x:{} y:{}",self.cam_look_x,self.cam_look_y);
        
        cam.dir =(
            (self.cam_look_x - 3.14/2.0).cos() * self.cam_look_y.cos(),
            self.cam_look_y.sin(),
            (self.cam_look_x - 3.14/2.0).sin() * self.cam_look_y.cos(),
        );

    }

    fn cam_cal(&self,s:&mut Scene){
        let cam = &mut s.cam;
        if cam.z > -3.0{
            cam.z = -3.0;
        }
        if cam.z < -43.0{
            cam.z = -43.0;
        }
        if cam.x > 43.0{
            cam.x = 43.0;
        }
        if cam.x < -43.0{
            cam.x = -43.0;
        }
    }

    fn ball_upadte(&mut self,win:&Winsdl,s:&mut Scene,dt:f32) -> bool{
        let mut rng = rand::thread_rng();

        let ob = s.objects.get_mut(BALL_POAINTER).unwrap();
        ob.x = self.ball.x;
        ob.y = self.ball.y;
        ob.z = self.ball.z;
        self.ball.update(dt);

        if self.ball.y < 5.0 && self.ball.z > 3.0{
            self.ball.velosty = 
            (-self.ball.velosty.0 * ((rng.gen::<f32>() - 0.5) * 4.0) + (rng.gen::<f32>() * 4.0),((rng.gen::<f32>() + 4.5) * 8.0)
            ,-(self.ball.velosty.2 * ((rng.gen::<f32>() - 0.5) * 4.0)).abs() - (rng.gen::<f32>() * 4.0));
        }

        if self.ball.z > 43.0 && self.ball.velosty.2 > 0.0{
            self.ball.velosty.2 = -self.ball.velosty.2;
        }
        if self.ball.z < -43.0 && self.ball.velosty.2 < 0.0{
            self.ball.velosty.2 = -self.ball.velosty.2;
        }
        if self.ball.x > 43.0 && self.ball.velosty.0 > 0.0{
            self.ball.velosty.0 = -self.ball.velosty.0;
        }
        if self.ball.x < -43.0 && self.ball.velosty.0 < 0.0{
            self.ball.velosty.0 = -self.ball.velosty.0;
        }

        if self.ball.y < 8.0 && self.ball.z < 3.0 && self.ball.z > -3.0 {
            self.ball.velosty = 
            (-self.ball.velosty.0,-self.ball.velosty.1,-self.ball.velosty.2);
            
            for _ in 0..5{
                self.ball.update(dt);
            }
        }

        if self.ball.y < -1.0 && self.ball.z < -3.0{
            return true;
        }

        match &mut self.shot {
            Some(shot) =>{
                shot.update(s,dt,&mut self.ball);
                if shot.time > SHOT_TIME_SEC{
                    self.shot = None;
                }
            },
            None => {
            if mouse_pressed_left(&win.event_pump){
                self.shot = Some(Shot::start(s,&mut self.ball))
            }},
            }
        false
    }

    fn upade_backgrund(&mut self,cam:&Camare,fb:&mut FrameBuffer){
        self.background_secne.cam = cam.clone();

        self.background_secne.cam.x = self.background_secne.cam.x/BACKGRUND_SLOOWNES;
        self.background_secne.cam.y = self.background_secne.cam.y/BACKGRUND_SLOOWNES;
        self.background_secne.cam.z = self.background_secne.cam.z/BACKGRUND_SLOOWNES;

        fb.bind();
        self.background_secne.draw();
        fb.unbind();
    }

    fn update_loss(&mut self,win:&Winsdl,s:&mut Scene)-> bool{
        if is_pressed(&win.event_pump,Scancode::Space){
            s.clear_objects();
            let (bs,ball) = DemoGameLogik::crate_game_world(s,win);
            self.ball = ball;
            self.background_secne = bs;
            s.sttinges.color_offset = 0.0;
            s.sttinges.color_senstivity = s.sttinges.color_senstivity * 4.0;
            return true;
        }
        false
    }
}

struct Ball{
    velosty:(f32,f32,f32),
    x:f32,
    y:f32,
    z:f32,
    r:f32,
}

impl Ball {
    
    fn call_ditanen(&self,other_ball:&Ball) -> bool{
        let dis = ((self.x - other_ball.x) * (self.x - other_ball.x)
                    +(self.y - other_ball.y) * (self.y - other_ball.y)
                + (self.z - other_ball.z) * (self.z - other_ball.z)).sqrt();
        return self.r + other_ball.r > dis;
    }

    fn dis(&self,other_ball:&Ball) -> f32{
        let dis = ((self.x - other_ball.x) * (self.x - other_ball.x)
                    +(self.y - other_ball.y) * (self.y - other_ball.y)
                + (self.z - other_ball.z) * (self.z - other_ball.z)).sqrt();
        return dis - self.r;
    }

    fn update(&mut self,dt:f32){
        self.x += self.velosty.0 * dt;
        self.y += self.velosty.1 * dt;
        self.z += self.velosty.2 * dt;
        self.velosty.1 -= BALL_VELOSTY_LOSE_PER_SEC * dt;
    }
}


const SHOT_TIME_SEC:f32 = 1.0;
const SHOT_MAX_DIS:f32 = 30.0;
const START_EFFECT:f32 = 50.0;
const END_EFFECT:f32 = 90.0;
struct Shot{
    ball_efected:(f32,f32,f32),
    time:f32,
    ball_start_vel:(f32,f32,f32),
    start_color_offset:f32,
}

impl Shot {
    fn start(s:&mut Scene,b:&mut Ball) -> Shot{
        
        let ball_start_vel = b.velosty;
        let mut ball_efected = b.velosty;

        let effect = (b.x - s.cam.x,b.y - s.cam.y,b.z - s.cam.z);
        let nor = (effect.0 * effect.0 + effect.1 * effect.1 + effect.2 * effect.2).sqrt();

        if nor < SHOT_MAX_DIS{
            let dis_effect = END_EFFECT + nor * ((END_EFFECT - START_EFFECT)/-SHOT_MAX_DIS);
            let effect = ((effect.0/nor)*dis_effect,(effect.1/nor)*dis_effect,(effect.2/nor)*dis_effect);
            ball_efected = effect;
        }

        Shot{
            ball_efected:ball_efected,
            time:0.0,
            ball_start_vel:ball_start_vel,
            start_color_offset:s.sttinges.color_offset,
        }
    }

    fn update(&mut self,s:&mut Scene,dt:f32,b:&mut Ball){
        self.time += dt;

        let t = self.time/SHOT_TIME_SEC;
        let t_left = 1.0 - t;

        b.velosty = (self.ball_efected.0 * t + self.ball_start_vel.0 * t_left
        ,self.ball_efected.1 * t + self.ball_start_vel.1 * t_left,
        self.ball_efected.2 * t + self.ball_start_vel.2 * t_left);
            
        s.sttinges.color_offset = self.start_color_offset-(((t*8.0-1.0).cbrt() + 1.0)/3.0);
    }
}