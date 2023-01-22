use std::io::Cursor;
use std::mem::swap;
use glium;
use std::path::Path;
use cgmath::Matrix4;
use glium::backend::Facade;
use glium::{Program, Surface, uniform};
use glium::texture::{SrgbTexture2d, Texture2dDataSource};
use glium::uniforms::{EmptyUniforms, UniformsStorage};
use crate::graphics::Vertex;
use crate::rect::{Rect, Rectangular};


type Uniforms <'a> = UniformsStorage<'a, &'a SrgbTexture2d, UniformsStorage<'a, [[f32; 4]; 4],
    EmptyUniforms>>;

pub struct Sprite {
    rect: Rect,
    texture: SrgbTexture2d,
    frames_h: u32,
    frames_v: u32,
    cur_frame: u32,
    layer: u32,
    hidden: bool,
    //components
}

impl Sprite {
    pub fn from(path:&Path, display: &glium::Display,
                   width: u32, height: u32) -> Sprite
    {
                        // Load the texture.
        let texture = {
            let img = image::open(path).unwrap().to_rgba16();
            let img_dim = img.dimensions();
            let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(),
                                                                         img_dim);

            SrgbTexture2d::new(display, img).unwrap()
        };
        let rect = Rect::from(width, height);
        return Sprite {rect, texture, frames_h:1, frames_v:1, cur_frame:1, layer:1, hidden:false};

    }
    pub fn set_layer(&mut self, layer: u32) {
        self.layer = layer;
    }
    pub fn at_layer(mut self, layer: u32) -> Sprite {
        self.layer = layer;
        self
    }
    pub fn set_frames(&mut self, frames_h: u32, frames_v: u32) {
        self.frames_h = frames_h;
        self.frames_v = frames_v;
    }
    pub fn with_frames(mut self, frames_h: u32, frames_v: u32) -> Sprite {
        self.frames_h = frames_h;
        self.frames_v = frames_v;
        self
    }
}
impl Rectangular for Sprite {
    fn get_rect(&self) -> &Rect {
        &self.rect
    }
    fn get_rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }
}


pub struct SpriteManager<'a> {
    display: &'a glium::Display,
    program: &'a glium::Program,
    perspective: [[f32; 4]; 4],
    draw_parameters: glium::draw_parameters::DrawParameters<'a>,
}

impl SpriteManager<'_> {
    pub fn from<'a>(display: &'a glium::Display, program: &'a glium::Program,
    screen_width: u32, screen_height: u32) -> SpriteManager<'a> {
        let perspective = {
            let matrix: Matrix4<f32> = cgmath::ortho(
                0.0,
                screen_width as f32,
                screen_height as f32,
                0.0,
                -1.0,
                1.0
            );
            Into::<[[f32; 4]; 4]>::into(matrix)
        };
        let draw_parameters = glium::DrawParameters {
            blend: glium::draw_parameters::Blend::alpha_blending(),
            .. Default::default()
        };
        SpriteManager {display, program, perspective, draw_parameters}
    }
    pub fn new_sprite(&self, path:&Path, width: u32, height: u32) -> Sprite
    {
        Sprite::from(path, self.display, width, height)
    }

    pub fn draw(&self, image: &Sprite, frame: &mut glium::Frame) {
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
            let vb = glium::VertexBuffer::empty_dynamic(self.display, 4).unwrap();
            // Creates an index buffer showing how the triangles would be made from the four points.
            let ib = glium::IndexBuffer::new(
                self.display,
                glium::index::PrimitiveType::TrianglesList,
                &ib_data
            ).unwrap();

            (vb, ib)
        };
        // Dynamically set the rectangle's vertices.
        {
            let rect = image.get_rect();
            let left = rect.left();
            let right = rect.left() + rect.width() as f64;
            let bottom = rect.bottom();
            let top = rect.bottom() - rect.height() as f64;
            //println!("left: {left}\nright: {right}\nbottom: {bottom}\ntop: {top}");
            let vb_data = vec![
                Vertex { position: [left, top] },
                Vertex { position: [right, top] },
                Vertex { position: [left, bottom] },
                Vertex { position: [right, bottom] }
            ];
            rect_vertices.write(&vb_data);
        }
        let uniforms = uniform! {
            projection: self.perspective,
            tex: &image.texture,
        };
        frame.draw(
                &rect_vertices,
                &rect_indices,
                self.program,
                &uniforms,
                &self.draw_parameters
        ).unwrap();
    }
}
