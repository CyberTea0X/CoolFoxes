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
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use engine::graphics::{Image, Vertex};
use engine::graphics::image::ImageManager;
use engine::Rect;

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
    let mut x = 200;
    let mut y = 200;
    // Load the texture.

    // Main event loop where all the drawing code is contained.
    event_loop.run_return(|event, _, control_flow|{
        let mut frame = display.draw();
        let display = &display;

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        control_flow.set_poll();
        x += 1;
        img.move_ip(x / 10, y);

        // Start with white background.
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        image_manager.draw(&img, &mut frame);
        frame.finish().unwrap();

        // Handles keyboard input.
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
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
    });
}