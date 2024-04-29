use sdl2::keyboard::Scancode;

use crate::{input::{is_pressed, move_mouse_to_center}, opengl_shit::{FrameBuffer, Texture}, Camare, Scene, SceneBackGround, SceneSttinges, Winsdl};

pub struct DemoGameLogik{
    //background_framebuffer:FrameBuffer,
    background_secne:Scene,
    velosty_y:f32,
}


impl DemoGameLogik{

    pub fn empty() -> DemoGameLogik{
        DemoGameLogik{
            //background_framebuffer:FrameBuffer::new(0,0),
            background_secne: Scene::new(SceneSttinges{
                max_rays:         0,
                min_dis_ray:      0.0,
                max_dis_ray:      0.0,
                color_senstivity: 0.0,
                color_offset:     0.0,
                colors_rgb:       [(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)],
                background: SceneBackGround::Color(0.0, 0.0, 0.0),
            }, Camare::new(0.0,0.0,0.0),0,0),
            velosty_y:0.0,
        }
    }

    pub fn new(s:&mut Scene,win:&Winsdl) -> DemoGameLogik{
        s.clear_objects();

        let k = s.add_object("evil_man");

        k.z = 3.0;
        k.angle_x = 3.14/2.0;

        win.sdl.mouse().show_cursor(false);
        _ = move_mouse_to_center(win);

        let mut bs = Scene::new(s.sttinges.clone(), s.cam.clone(), s.get_scene_width(),s.get_scene_height());
        bs.add_folder_to_model("src/objects");
        bs.update_shader();
        bs.sttinges.background = SceneBackGround::Color(1.0, 1.0, 1.0);
        bs.sttinges.color_senstivity = 0.003;
        
        let d = bs.add_object("death");
        d.angle_z = 3.14/2.0;

        let fb = FrameBuffer::new(s.get_scene_width(),s.get_scene_height());
        fb.get_texture().load("Screenshot 2024-04-18 012040.jpg");
        s.sttinges.background = SceneBackGround::FrameBuffer(fb);
        
        print!("{}:{}",bs.objects.len(),bs.objects_models.len());
        DemoGameLogik{
            //background_framebuffer:fb,
            velosty_y:0.0,
            background_secne: bs,
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

            if cam.angle_y >= 3.14/4.0{
                cam.angle_y = 3.14/4.0
            }
            else if cam.angle_y <= -3.14/4.0{
                cam.angle_y = -3.14/4.0
            }
        }

        match &mut s.sttinges.background {
            SceneBackGround::FrameBuffer(fb) => {
                self.upade_backgrund(cam,fb)
            },
            SceneBackGround::Image(_) => {},
            SceneBackGround::Color(_, _, _) => {},
            SceneBackGround::ContinuationOfRay(_, _) => {},
        }
    }

    fn upade_backgrund(&mut self,cam:&Camare,fb:&mut FrameBuffer){
        self.background_secne.cam = cam.clone();
        fb.bind();
        self.background_secne.draw();
        fb.unbind();
    }
}