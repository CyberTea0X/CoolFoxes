extern crate cgmath;
extern crate glium;
extern crate image;

use std::path::Path;
use std::time::{Duration, Instant};
use cgmath::Vector2;

use glium::glutin;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use glium::Surface;
use measurements::Mass;
use engine::component::traits::Composite;

use engine::graphics::sprite::SpriteManager;
use engine::graphics::sprite::SpriteGroup;
use engine::group::SomeGroup;
use engine::programs::ProgramManager;
use engine::rect::Rectangular;
use engine::time::Clock;
use engine::misc_traits::named::Named;
use engine::physics::components::PhysicsComponent;

const SCREEN_WIDTH: u32 = 1224;
const SCREEN_HEIGHT: u32 = 768;

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title(format!("Cool foxes"));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let rect_program = ProgramManager::rect_drawing_program(&display);
    let sprite_manager = SpriteManager::from(&display, &rect_program,
                                             SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut sprites = SpriteGroup::new();
    let gravity = 9.8;
    sprites.push(sprite_manager.build_sprite(Path::new("./assets/images/fox.png"), 0.10)
        .with_position(0, 0)
        .named("fox")
        .with_component(
            PhysicsComponent::new(Mass::from_kilograms(50.0), gravity,
            Some(Vector2::new(60.0, 10.0)))));
    sprites.push(sprite_manager.build_sprite(Path::new("./assets/images/target.png"), 0.10)
        .with_position(SCREEN_WIDTH-150, 0)
        .named("target")
        .with_component(
            PhysicsComponent::new(Mass::from_kilograms(100.0), gravity,
            Some(Vector2::new(-100.0, 10.0)))));
    sprites.push(sprite_manager.build_bg(Path::new("./assets/images/bg.png"))
        .with_position(0, 768));
    println!("{:#?}", sprites);
    let mut dt;
    let fps = 60;
    let mut event_handling_start: Instant;
    let mut frame_handling_start: Instant;
    let mut running = true;
    let mut clock = Clock::new();
    let start_time = Instant::now();
    let mut wait = Duration::from_secs(1);
    while running {
        let mut frame = display.draw();
        frame_handling_start = Instant::now();
        dt = clock.get_time().as_millis() as u32;
        //println!("{}", dt);
        // Start with white background.
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        sprites.call(|mut sprite| {
            sprite = sprite.updated(dt);
            sprite_manager.draw(&sprite, &mut frame);
            if start_time.elapsed() >= wait {
                println!("{:?}", sprite)
            }
            sprite
        }
        );
        if start_time.elapsed() >= wait {
            wait += Duration::from_secs(1);
        }
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