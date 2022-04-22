#![allow(non_snake_case)]

mod components;
mod core;

use std::f64::consts::FRAC_1_SQRT_2;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{World};
use rand::Rng;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::{Scancode};
use sdl2::rect::{Rect};
use crate::components::{Acceleration, Mob, Movement, TypeMovement, Player, TexRect};
use crate::core::{Renderer, spawn_mob, TARGET_FPS};


fn main() {
    let mut core = Renderer::new("a start pathing");

    let mut event_pump = core.sdl_context.event_pump().unwrap();

    let texture_creator = core.EKRAN.texture_creator();
    let texture_sprite_sheet = texture_creator.load_texture("./assets/sprites.png").unwrap();

    const VEL: f64 = 10.0;
    const SPAWN_X: f64 = 1280.0 / 2.0;
    const SPAWN_Y: f64 = 720.0 / 2.0;

    let mut world = World::new();



    let player = world.spawn()
                      .insert(Player {})
                      .insert(Movement { movetype: TypeMovement::Player, x: SPAWN_X, y: SPAWN_Y })
                      .insert(Acceleration { x: 0.0, y: 0.0 })
                      .insert(TexRect { srs: Rect::new(16, 16, 16, 17), pos: Rect::new(0, 0, 16 * 3, 17 * 3) })
                      .id();


    for i in 0..10 {
        spawn_mob(&mut world, i);
    }


    let player_movem_ptr = world.get::<Movement>(player).unwrap() as *const Movement;


    let mut movement_update = world.query::<(&mut TexRect, &mut Movement, &mut Acceleration)>();
    let mut evnent_movement = world.query::<(&mut Acceleration, &Player)>();
    let mut render = world.query::<&TexRect>();
    let mut mob_movement_mut = world.query::<(&mut Movement, &Mob, Entity)>();
    let mut mob_movement = world.query::<(&Movement, &Mob, Entity)>();

    let mut timmer = 0.0;
    let mut timmer_max = 10.0;

    let mut enable_ai = true;
    let mut is_running = true;
//--------- LOOP
    while is_running {
        core.fps_ctrl.start();
        core.EKRAN.set_draw_color((10, 10, 30));
        core.EKRAN.clear();

        timmer += core.fps_ctrl.dt;

        // dbg!(timmer);
        // dbg!(core.fps_ctrl.ttime.ticks() % 5000 <= 100);
        if timmer > timmer_max {
            spawn_mob(&mut world, rand::thread_rng().gen_range(0..=1));

            if timmer_max > 1.0 {
                timmer_max -= 1.0;
            }
            timmer = 0.0;
        }
        let keyb = event_pump.keyboard_state();
        for (mut acceleration, _) in evnent_movement.iter_mut(&mut world) {
            if keyb.is_scancode_pressed(Scancode::A) {
                acceleration.x -= VEL;
            }
            if keyb.is_scancode_pressed(Scancode::D) {
                acceleration.x += VEL;
            }
            if keyb.is_scancode_pressed(Scancode::W) {
                acceleration.y -= VEL;
            }
            if keyb.is_scancode_pressed(Scancode::S) {
                acceleration.y += VEL;
            }

            if acceleration.x != 0.0 && acceleration.y != 0.0 {
                acceleration.x *= FRAC_1_SQRT_2;
                acceleration.y *= FRAC_1_SQRT_2;
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => { is_running = false; },
                Event::KeyDown { scancode, .. } => {
                    match scancode.unwrap() {
                        Scancode::F1 => {
                            enable_ai = !enable_ai;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if enable_ai {
            unsafe {
                for (mut movem1, _, entt1) in mob_movement_mut.iter_unchecked(&world) {
                    for (movem2, _, entt2) in mob_movement.iter(&world) {
                        if entt1 != entt2 {
                            let x = (movem1.x - movem2.x) as f64;
                            let y = (movem1.y - movem2.y) as f64;

                            let dist = (x * x + y * y).sqrt();

                            if dist < (16.0 * 3.0) {
                                let normalized = if dist != 0.0 { (x / dist, y / dist) } else { (x, y) };

                                movem1.x += normalized.0 * 5.0;
                                movem1.y += normalized.1 * 5.0;
                                // movem2.x -= normalized.0 * 5.0;
                                // movem2.y -= normalized.1 * 5.0;
                            }
                        }
                    }
                }
            }
        }


        for (textrect, mut movem, accs) in movement_update.iter_mut(&mut world) {
            match movem.movetype {
                TypeMovement::Player => movem.player_movement(textrect, accs, core.fps_ctrl.dt *
                    TARGET_FPS),
                TypeMovement::Mob => {
                    if enable_ai {
                        movem.bandit_movement(textrect, accs, core.fps_ctrl.dt * TARGET_FPS, unsafe { &*player_movem_ptr });
                    }
                },
            }
        }

        {
            let mut render = render.iter(&world).collect::<Vec<_>>();
            render.sort_by(|a, b| a.pos.center().y().cmp(&b.pos.center().y()));

            for tex in render {
                core.EKRAN.copy(&texture_sprite_sheet, tex.srs, tex.pos).unwrap();
            }
        }

        core.EKRAN.present();
        core.fps_ctrl.end();
    }
}
