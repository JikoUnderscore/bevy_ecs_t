use std::time;
use bevy_ecs::prelude::World;
use rand::Rng;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::{Acceleration, Mob, Movement, TexRect, TypeMovement};

pub struct Renderer {
    pub sdl_context: Sdl,
    pub EKRAN: WindowCanvas,
    pub fps_ctrl: FpsCapDeltaTime,
}

impl Renderer {
    pub fn new(title: &str) -> Renderer {

        // init systems.
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // create a window.
        let mut win = video_subsystem.window(title, 1280, 720);
        let window = win.position_centered()
                        .resizable()
                        .build()
                        .unwrap();


        // get the canvas
        let mut ekran = window.into_canvas().build().unwrap();
        ekran.set_logical_size(1280, 720).unwrap();


        Renderer {
            EKRAN: ekran,
            fps_ctrl: FpsCapDeltaTime::new(60),
            sdl_context,
        }
    }
}

pub const TARGET_FPS: f64 = 60.0;

pub struct StartTimer {
    start_time: time::Instant
}

impl StartTimer {
    pub fn ticks(&self) -> u32 {
        return self.start_time.elapsed().as_millis() as u32;
    }
}


pub struct FpsCapDeltaTime {
    last_time: time::Instant,
    pub ttime: StartTimer,
    frame_delay: u64,
    pub set_fps: f64,
    cap_frame_start: time::Instant,
    pub dt: f64,
}

impl FpsCapDeltaTime {
    pub fn new(fps: u64) -> Self {
        let last_time = time::Instant::now();

        Self {
            last_time,
            ttime: StartTimer { start_time: time::Instant::now() },
            frame_delay: (1000 / fps),
            set_fps: fps as f64,
            cap_frame_start: time::Instant::now(),
            dt: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.cap_frame_start = time::Instant::now();
        self.dt = 1.0 / (1.0 / self.last_time.elapsed().as_secs_f64());
        // println!("FPS: {} | set fps {} | dt {} ", 1.0 / self.dt , self.set_fps, self.dt);
        // println!("{}", (self.dt * self.set_fps));
        self.last_time = time::Instant::now();
    }

    pub fn end(&mut self) {
        let cap_frame_end = self.cap_frame_start.elapsed().as_millis() as u64;
        if cap_frame_end < self.frame_delay {
            std::thread::sleep(time::Duration::from_millis(self.frame_delay - cap_frame_end));
        }
    }
}


fn random_pos_xy(i: i32) -> (i32, i32) {
    let rand_x = if i % 2 == 0 {
        if rand::thread_rng().gen::<f32>() > 0.5 { -17 } else { 1280+17 }
    } else {
        rand::thread_rng().gen_range(-17..1280+17)
    };
    let rand_y = if i % 2 == 0 {
        rand::thread_rng().gen_range(-17..720+17)
    } else {
        if rand::thread_rng().gen::<f32>() > 0.5 { -17 } else { 720+17 }
    };

    return (rand_x, rand_y);
}



pub fn spawn_mob(world: &mut World, i: i32){
    static MOB_LIST: [(i32, i32, u32, u32); 3] = [(0, 0, 16, 16), (16, 0, 16, 16), (0, 16, 16, 17)];

    let r = rand::thread_rng().gen_range(0..3);
    let mob_src = Rect::from(MOB_LIST[r]);
    let (rand_x, rand_y) = random_pos_xy(i);

    world.spawn()
         .insert(Mob {})
         .insert(Movement { movetype: TypeMovement::Mob, x: rand_x as f64, y: rand_y as f64 })
         .insert(Acceleration { x: 0.0, y: 0.0 })
         .insert(TexRect { srs: mob_src, pos: Rect::new(rand_x, rand_y, mob_src.width() * 3, mob_src.width() * 3) });

}