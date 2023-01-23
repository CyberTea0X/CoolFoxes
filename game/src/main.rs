#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

use cgmath::{Vector2, Matrix4};
use glium::glutin;
use glium::{Surface};
use std::io::Cursor;
use glium::glutin::dpi::PhysicalSize;
use std::path::Path;
use std::time::{Duration, Instant};
use glium::glutin::platform::run_return::EventLoopExtRunReturn;

use engine::graphics::{Sprite, Vertex};
use engine::graphics::sprite::SpriteManager;
use engine::group::Group;
use engine::Rect;
use engine::time::Clock;
use engine::rect::Rectangular;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title(format!("Hello world!"));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let rect_program = Rect::drawing_program(&display);
    let sprite_manager = SpriteManager::from(&display, &rect_program,
                                             SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut spr1 = sprite_manager.new_sprite(Path::new("fox.png"), 300, 300)
        .with_position(0, 768);
    let mut spr2 = sprite_manager.new_sprite(Path::new("wolf.png"), 500, 300)
        .with_position(1024, 768);
    let mut dt = 0;
    let fps = 60;
    let mut event_handling_start: Instant;
    let mut frame_handling_start: Instant;
    let mut running = true;
    let mut clock = Clock::new();
    while running {
        let mut frame = display.draw();
        frame_handling_start = Instant::now();
        println!("delta time: {}", dt);
        dt = clock.get_time().as_millis();
        spr1.move_by(1, 0);
        spr2.move_by(-1, 0);
        if spr1.x() > 1024.0 {
            spr1.move_ip(Some(0), None);
        }
        if spr2.right() < 0.0 {
            spr2.move_ip(Some(1024), None);
        }

        // Start with white background.
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        sprite_manager.draw(&spr1, &mut frame);
        sprite_manager.draw(&spr2, &mut frame);
        frame.finish().unwrap();

        // Handles keyboard input.
        event_handling_start = Instant::now();

        // Большая и страшная обработка событий и времени между кадрами
        event_loop.run_return(|event, _, control_flow|{
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        running = false;
                        return;
                    },
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }
            if (event_handling_start.elapsed() + frame_handling_start.elapsed()) >
                Duration::from_secs(1) / fps {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            }
            let wait_duration = (Duration::from_secs(1) / fps)
                .checked_sub(event_handling_start.elapsed() + frame_handling_start.elapsed())
                .unwrap_or(Duration::from_secs(0));
            let next_frame_time = Instant::now() + wait_duration;
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
    }
}