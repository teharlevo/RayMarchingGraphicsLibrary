use sdl2::keyboard::Scancode;

use crate::{input::{is_pressed, move_mouse_to_center}, opengl_shit::FrameBuffer, Camare, Scene, SceneBackGround, Winsdl};

const BACKGRUND_SLOOWNES:f32 = 5.0;
const SENSTIVITY:f32 = 0.08;
const PLAYER_SPEED:f32 = 30.0;
const VELOSTY_LOSE_PER_SEC:f32 = 3.0;
const JUMP_FORCE:f32 = 1.5;

const BALLS_NUM:i32 = 5;
const BALLS_POINTER:usize = 2;
pub struct DemoGameLogik{
    //hund_secne:Scene,
    background_secne:Scene,
    velosty_y:f32,
    cam_look_x:f32,
    cam_look_y:f32,
    balls:Vec<Ball>,
}


impl DemoGameLogik{

    pub fn new(s:&mut Scene,win:&Winsdl) -> DemoGameLogik{
        //let mut hs_s = s.sttinges.clone();
        //hs_s.background = SceneBackGround::FrameBuffer(FrameBuffer::new(s.get_scene_width(),s.get_scene_height()));
        //let mut hs = Scene::new(hs_s, Camare::new(0.0,0.0,0.0),s.get_scene_width(),s.get_scene_height());

        s.clear_objects();

        win.sdl.mouse().show_cursor(false);
        _ = move_mouse_to_center(win);

        let mut bs = Scene::new(s.sttinges.clone(), s.cam.clone(), s.get_scene_width(),s.get_scene_height());
        bs.add_folder_to_model("src/objects");
        bs.update_shader();
        bs.sttinges.background = SceneBackGround::Color(1.0, 1.0, 1.0);
        bs.sttinges.color_senstivity = 0.003;
        bs.sttinges.max_dis_ray = 1000.0;
        bs.sttinges.show_above_min_dis_errors = true;
        bs.sttinges.max_rays = 60;

        //let k = hs.add_object("hund");
        //k.angle_y = 3.14;
        //k.angle_z = 3.14/2.0;

        let k = s.add_object("evil_man");

        
        k.z = 3.0;
        k.angle_x = 3.14/2.0;

        let k = s.add_object("floor");
        k.y = -2.0;
        
        let d = bs.add_object("death");
        d.angle_z = 3.14/2.0;

        let fb = FrameBuffer::new(s.get_scene_width(),s.get_scene_height());
        s.sttinges.background = SceneBackGround::FrameBuffer(fb);
        let mut v = vec![];
        for i in 0..BALLS_NUM {
            v.push(Ball{
                x: (i*5) as f32,
                y: (i*5) as f32,
                z: (i*2) as f32,
                r: (i*2 + 1) as f32,
            });
            _ = s.add_object("sphere");
        }
        DemoGameLogik{
            //background_framebuffer:fb,
            //hund_secne:hs, 
            velosty_y:0.0,
            background_secne: bs,
            cam_look_x:0.0,
            cam_look_y:0.0,
            balls:v,
        }
    }

    pub fn update(&mut self,s:&mut Scene,win:&Winsdl,dt:f32) -> bool{

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

        if !is_pressed(&win.event_pump,Scancode::Escape) {
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

        match &mut s.sttinges.background {
            SceneBackGround::FrameBuffer(fb) => {
                self.upade_backgrund(cam,fb)
            },
            SceneBackGround::Image(_) => {},
            SceneBackGround::Color(_, _, _) => {},
            SceneBackGround::ContinuationOfRay(_, _) => {},
        }

        {
            let ball_1 = self.balls.get_mut(0).unwrap();
            if is_pressed(&win.event_pump, Scancode::Up) {
                ball_1.z += 20.0 * dt;
            }
            if is_pressed(&win.event_pump, Scancode::Down) {
                ball_1.z -= 20.0 * dt;
            }
            if is_pressed(&win.event_pump, Scancode::Left) {
                ball_1.y += 20.0 * dt;
            }
            if is_pressed(&win.event_pump, Scancode::Right) {
                ball_1.y -= 20.0 * dt;
            }
            if is_pressed(&win.event_pump, Scancode::Z) {
                ball_1.x += 20.0 * dt;
            }
            if is_pressed(&win.event_pump, Scancode::X) {
                ball_1.x -= 20.0 * dt;
            }
            
            let ball_1 = self.balls.get(0).unwrap();
            let mut i: usize = 0;
            for b in &self.balls {
                let ob = s.objects.get_mut(i + BALLS_POINTER);
                match ob {
                    Some(ob) => {
                        ob.x = b.x;
                        ob.y = b.y;
                        ob.z = b.z;
                        ob.scale = b.r;
                        if i != 0 && b.call_ditanen(ball_1) {
                            println!("{i}");
                        }
                    }
                    None => {
                        println!("wtf");
                    }
                }
                i = i + 1;
            }
        }

        //self.hund_secne.sttinges.color_offset = ((cam.x * cam.x + cam.y * cam.y + cam.z * cam.z).sqrt())*s.sttinges.color_senstivity;
        //match &mut self.hund_secne.sttinges.background {
        //    SceneBackGround::FrameBuffer(fb) => {
        //        fb.bind();
        //        s.draw();
        //        fb.unbind();
        //    },
        //    SceneBackGround::Image(_) => {},
        //    SceneBackGround::Color(_, _, _) => {},
        //    SceneBackGround::ContinuationOfRay(_, _) => {},
        //}
        //self.hund_secne.draw();
        s.draw();

        if is_pressed(&win.event_pump,Scancode::Escape){
            s.clear();
            return true;
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
}

//enum GameMode {
//    Noemal,
//    Pause(f32),
//}

struct Ball{
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
}