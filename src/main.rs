#![allow(non_snake_case)]

use std::collections::{VecDeque};
use std::f64::consts::FRAC_1_SQRT_2;
use std::time;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Component, World};
use sdl2::render::WindowCanvas;
use sdl2::{Sdl, TimerSubsystem};
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::{KeyboardState, Scancode};
use sdl2::mouse::MouseButton;
use sdl2::rect::{Point, Rect};


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
        let ekran = window.into_canvas().build().unwrap();
        // ekran.set_logical_size(1280, 720).unwrap();


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

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Movement {
    x: f64,
    y: f64,
}

#[derive(Component)]
struct Acceleration {
    x: f64,
    y: f64,
}

#[derive(Component)]
struct TexRect {
    srs: Rect,
    pos: Rect,
}


fn main() {
    let mut core = Renderer::new("a start pathing");

    let mut event_pump = core.sdl_context.event_pump().unwrap();

    let texture_creator = core.EKRAN.texture_creator();
    let texture_sprite_sheet = texture_creator.load_texture("./assets/SS.png").unwrap();

    const VEL: f64 = 10.0;

    let mut world = World::new();


    let player = world.spawn()
                      .insert(Player{})
                      .insert(Movement{ x: 0.0, y: 0.0 })
                      .insert(Acceleration{ x: 0.0, y: 0.0 })
                      .insert(TexRect{ srs: Rect::new(0,24,24,24), pos: Rect::new(0,0,24*3,24*3) })
                      .id();
    let player2 = world.spawn()
                       .insert(Player{})
                       .insert(Movement{ x: 24.0*3.0, y: 0.0 })
                       .insert(Acceleration{ x: 0.0, y: 0.0 })
                       .insert(TexRect{ srs: Rect::new(0,24,24,24), pos: Rect::new(0,0,24*3,24*3) })
                       .id();


    let mut movement_update = world.query::<(&mut TexRect, &mut Movement, &mut Acceleration)>();
    let mut remove_ent = world.query::<Entity>();
    let mut evnent_movement = world.query::<(&mut Acceleration, &Player)>();
    let mut render = world.query::<(&TexRect)>();

    let mut is_running = true;

//--------- LOOP
    while is_running {
        core.fps_ctrl.start();
        core.EKRAN.set_draw_color((0, 0, 0));
        core.EKRAN.clear();

        let keyb = event_pump.keyboard_state();
        for (mut acceleration, _) in evnent_movement.iter_mut(&mut world) {
            if keyb.is_scancode_pressed(Scancode::A) {
                acceleration.x -= VEL ;
            }
            if keyb.is_scancode_pressed(Scancode::D) {
                acceleration.x += VEL ;
            }
            if keyb.is_scancode_pressed(Scancode::W) {
                acceleration.y -= VEL ;
            }
            if keyb.is_scancode_pressed(Scancode::S) {
                acceleration.y += VEL ;
            }

            // if acceleration.x != 0.0 && acceleration.y != 0.0 {
            //     acceleration.x *= FRAC_1_SQRT_2;
            //     acceleration.y *= FRAC_1_SQRT_2;
            // }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => { is_running = false; },
                Event::KeyDown { scancode, .. } => {
                    match scancode.unwrap() {
                        Scancode::Num1 | Scancode::O => {
                            for ent in remove_ent.iter(&world){
                                world.despawn(ent);
                                break;
                            }

                        }
                        Scancode::Num2 | Scancode::P => {
                            world.spawn()
                                 .insert(Player{})
                                 .insert(Movement{ x: 0.0, y: 0.0 })
                                 .insert(Acceleration{ x: 0.0, y: 0.0 })
                                 .insert(TexRect{ srs: Rect::new(0,24,24,24), pos: Rect::new(0,0,24*3,24*3) })
                                 .id();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }





        for (mut textect, mut movem, mut accs) in movement_update.iter_mut(&mut world) {
            movem.x += accs.x.clone() * core.fps_ctrl.dt * TARGET_FPS;
            movem.y += accs.y.clone() * core.fps_ctrl.dt * TARGET_FPS;

            // println!("{} {}", movem.x, movem.y);
            textect.pos.set_x(movem.x as i32);
            textect.pos.set_y(movem.y as i32);

            accs.x = 0.0;
            accs.y = 0.0;
        }

        for tex in render.iter(&world) {
            core.EKRAN.copy(&texture_sprite_sheet, tex.srs, tex.pos).unwrap();
        }


        core.EKRAN.present();
        core.fps_ctrl.end();
    }
}
