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
use crate::rect::Rect;


type Uniforms <'a> = UniformsStorage<'a, &'a SrgbTexture2d, UniformsStorage<'a, [[f32; 4]; 4],
    EmptyUniforms>>;

pub struct Image {
    rect: Rect,
    pub texture: SrgbTexture2d,
}

impl Image {
    pub fn from(path:&Path, perspective: [[f32; 4]; 4], display: &glium::Display,
                   width: u32, height: u32) -> Image
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
        return Image {rect, texture};

    }
    pub fn from_rect(rect:glium::Rect) {}
    pub fn width(&self) -> u32 {
        self.rect.width()
    }
    pub fn height(&self) -> u32 {
        self.rect.height()
    }
    pub fn get_rect(&self) -> &Rect {
        &self.rect
    }
    pub fn move_ip<T: Into<f64>>(&mut self, x: Option<T>, y: Option<T>) {
        self.rect.move_ip(x, y);
    }
    pub fn move_by<A: Into<f64>, B: Into<f64>>(&mut self, x: A, y: B) {
        self.rect.move_by(x, y);
    }
    pub fn with_position<T: Into<f64>>(mut self, x: T, y: T) -> Image {
        self.rect.move_ip(Some(x), Some(y));
        self
    }
}


pub struct ImageManager<'a> {
    display: &'a glium::Display,
    program: &'a glium::Program,
    perspective: [[f32; 4]; 4],
    draw_parameters: glium::draw_parameters::DrawParameters<'a>,
}

impl ImageManager <'_> {
    pub fn from<'a>(display: &'a glium::Display, program: &'a glium::Program,
    screen_width: u32, screen_height: u32) -> ImageManager<'a> {
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
        ImageManager {display, program, perspective, draw_parameters}
    }
    pub fn build(&self, path:&Path, width: u32, height: u32) -> Image
    {
        Image::from(path, self.perspective, self.display, width, height)
    }

    pub fn draw(&self, image: &Image, frame: &mut glium::Frame) {
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
