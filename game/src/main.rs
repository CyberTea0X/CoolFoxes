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

use engine::graphics::{Image, Vertex};
use engine::graphics::image::ImageManager;
use engine::Rect;
use engine::time::Clock;

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
    let image_manager = ImageManager::from(&display, &rect_program,SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut img = image_manager.build(Path::new("fox.png"), 300, 300);
    let mut img2 = image_manager.build(Path::new("wolf.png"), 500, 300);
    let mut x = 0;
    let mut y = 768;
    let mut x2 = 1024;
    let mut y2 = 768;
    let mut dt = 0;
    let mut fps = 60;
    let mut event_handling_start: Instant;
    let mut running = true;
    let mut clock = Clock::new();
    while running {
        let mut frame = display.draw();
        println!("{}", dt);
        dt = clock.get_time().as_millis();
        x += 1;
        x2 -= 1;
        if x > 1024 as i32 {
            x = 0
        }
        if x2 < 0 as i32 {
            x2 = 1024
        }
        img.move_ip(x, y);
        img2.move_ip(x2, y2);

        // Start with white background.
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        image_manager.draw(&img, &mut frame);
        image_manager.draw(&img2, &mut frame);
        frame.finish().unwrap();

        // Handles keyboard input.
        event_handling_start = Instant::now();

        // Большая и ужасная обработка событий
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
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            if event_handling_start.elapsed() > (Duration::from_secs(1) / 60) {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }

    });
    }
}