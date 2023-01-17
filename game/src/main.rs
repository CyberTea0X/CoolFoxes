#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

use cgmath::{Vector2, Matrix4};
use glium::glutin;
use glium::{Surface};
use std::io::Cursor;
use glium::glutin::dpi::PhysicalSize;

#[derive(Clone, Copy)]
struct Vertex {
    // The fields in Vertex are usually there
    // to be passed into the shader file.
    position: [f32; 2],
}

// This line implements the Vertex using a macro inside glium.
// Don't forget to include all of the fields as parameters otherwise
// glium won't pass those into the shader.
implement_vertex!(Vertex, position);

const VERTEX_SHADER: &'static str = r#"
    #version 140
    // Input parameter from the Vertex struct.
    in vec2 position;
    // Uniform parameter passed in from the frame.draw() call.
    uniform mat4 projection;
    // Output texture coordinates that gets passed into the fragment shader.
    out vec2 v_tex_coords;
    void main() {
        // In order to return the texture coordinate for a specific
        // vertex we have to know what vertex is currently being passed in.
        // We do this through gl_VertexID which increments with every vertex passed in.
        // We can figure out the rectangle specific index from the vertex id by modding it
        // by 4. Example: if a vertex has id 16, then it is the first vertex of the fourth
        // rectangle being drawn. 16 % 4 == 0 which correctly returns the first index.
        if (gl_VertexID % 4 == 0) { // First vertex
            v_tex_coords = vec2(0.0, 1.0);
        } else if (gl_VertexID % 4 == 1) { // Second vertex
            v_tex_coords = vec2(1.0, 1.0);
        } else if (gl_VertexID % 4 == 2) { // Third vertex
            v_tex_coords = vec2(0.0, 0.0);
        } else { // Fourth vertex
            v_tex_coords = vec2(1.0, 0.0);
        }
        gl_Position = projection * vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    #version 140
    // Input texture coordinates passed from the vertex shader.
    in vec2 v_tex_coords;
    // Outputs the color for the specific fragment.
    out vec4 color;
    // Uniform parameter passed in from the frame.draw() call.
    uniform sampler2D tex;
    void main() {
        // Applies a texture to the rectangle.
        color = texture(tex, v_tex_coords);
    }
"#;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title(format!("Hello world!"));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Load the texture.
    let texture = {
        let img = image::load(
            Cursor::new(&include_bytes!("../fox.png")[..]),
            image::ImageFormat::Png
        ).unwrap().to_rgba16();

        let img_dim = img.dimensions();
        let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dim);

        glium::texture::Texture2d::new(&display, img).unwrap()
    };

    // Before we can draw the rectangle we have to
    // tell OpenGL what a rectangle is. All OpenGL needs
    // to know is that a rectangle is four vertexes (points)
    // and that you can make two triangles from the four points.
    let (rect_vertices, rect_indices) = {
        // Data specifying how triangles would be made from the 4 points.
        // The first triangle consists of vertexes 0, 1, and 2,
        // while the second triangle consists of 1, 3, 2.
        //
        // 0      1
        // +------+
        // |    / |
        // |  /   |
        // |/     |
        // +------+
        // 2      3
        let ib_data: Vec<u16> = vec![0, 1, 2, 1, 3, 2];

        // Creates a dynamic vertex buffer with four points.
        // Dynamic means that the actual vertexes will be specified later which means
        // the size of our rectangle is not fixed.
        let vb = glium::VertexBuffer::empty_dynamic(&display, 4).unwrap();
        // Creates an index buffer showing how the triangles would be made from the four points.
        let ib = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &ib_data
        ).unwrap();

        (vb, ib)
    };

    // Create a program from the two shaders.
    // A "program" is just a bunch of shaders so you can have multiple programs
    // for drawing different things.
    let rect_program = glium::Program::from_source(
        &display,
        VERTEX_SHADER,
        FRAGMENT_SHADER,
        None
    ).unwrap();

    let perspective = {
        let matrix: Matrix4<f32> = cgmath::ortho(
            0.0,
            SCREEN_WIDTH as f32,
            SCREEN_HEIGHT as f32,
            0.0,
            -1.0,
            1.0
        );
        Into::<[[f32; 4]; 4]>::into(matrix)
    };

    let rect_size = Vector2 {
        x: 300.0,
        y: 300.0,
    };

    let mut rect_position = Vector2 {
        x: (SCREEN_WIDTH / 2) as f32,
        y: (SCREEN_HEIGHT / 2) as f32,
    };

    // Main event loop where all the drawing code is contained.
    event_loop.run(move |event, _, control_flow|{
        let mut frame = display.draw();

        // Start with white background.
        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        // Dynamically set the rectangle's vertices.
        {
            let left = rect_position.x - rect_size.x / 2.0;
            let right = rect_position.x + rect_size.x / 2.0;
            let bottom = rect_position.y + rect_size.y / 2.0;
            let top = rect_position.y - rect_size.y / 2.0;
            let vb_data = vec![
                Vertex { position: [left, top] },
                Vertex { position: [right, top] },
                Vertex { position: [left, bottom] },
                Vertex { position: [right, bottom] }
            ];
            rect_vertices.write(&vb_data);
        }

        // Draw the rectangle.
        {
            // Uniform parameters to pass into the shaders.
            let uniforms = uniform! {
                projection: perspective,
                tex: &texture,
            };

            frame.draw(
                &rect_vertices,
                &rect_indices,
                &rect_program,
                &uniforms,
                &Default::default()
            ).unwrap();
        }

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
    })
}