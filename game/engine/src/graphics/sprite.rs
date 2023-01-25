use std::path::Path;

use cgmath::{Matrix4, Point2};
use glium;
use glium::{BlendingFunction, Surface, uniform};
use glium::draw_parameters::LinearBlendingFactor;
use glium::glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use glium::texture::SrgbTexture2d;

use crate::graphics::Vertex;
use crate::loader::TextureLoader;
use crate::rect::{Rect, Rectangular};
use crate::traits::graphics::{FrameList, Layered};

/// Спрайт - это текстура и квадрат, в котором эта текстура рисуется.
/// У спрайта есть слой, на котором он рисуется.
pub struct Sprite {
    rect: Rect,
    texture: SrgbTexture2d,
    frames_h: u32,
    frames_v: u32,
    _cur_frame: u32,
    layer: u32,
    _hidden: bool,
    //components
}

impl Sprite {
    pub fn new (rect: Rect, texture: SrgbTexture2d, frames_h: u32, frames_v: u32, _cur_frame: u32, layer: u32, _hidden: bool) -> Self {
        Sprite { rect, texture, frames_h, frames_v, _cur_frame, layer, _hidden }
    }
    pub fn updated(self) -> Sprite {
        return self
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

impl Layered for Sprite {
    fn get_layer(&self) -> u32 {
        self.layer
    }
    fn get_layer_mut(&mut self) -> &mut u32 {
        &mut self.layer
    }
}

impl FrameList for Sprite {
    fn get_frames(&self) -> (u32, u32) {
        (self.frames_h, self.frames_v)
    }
    fn get_frames_mut(&mut self) -> (&mut u32, &mut u32) {
        (&mut self.frames_h, &mut self.frames_v)
    }
}

/// SpriteManager занимается отрисовкой любых спрайтов на экране. Упрощает отрисовку.
pub struct SpriteManager<'a> {
    display: &'a glium::Display,
    program: &'a glium::Program,
    perspective: [[f32; 4]; 4],
    draw_parameters: glium::draw_parameters::DrawParameters<'a>,
    screen_size: PhysicalSize<u32>,
    screen_ratio: f64,
}

impl SpriteManager<'_> {
    /// Создаёт новый экземпляр SpriteManager из ссылки на дисплей, прогромму для рисования,
    /// также требует параметры экрана.
    pub fn from<'a>(display: &'a glium::Display, program: &'a glium::Program,
    screen_width: u32, screen_height: u32) -> SpriteManager<'a> {
        SpriteManager {
            display,
            program,
            perspective: SpriteManager::perspective_default(screen_width, screen_height),
            draw_parameters: SpriteManager::draw_parameters_default(),
            screen_size: PhysicalSize::new(screen_width, screen_height),
            screen_ratio: screen_width as f64 / screen_height as f64,
        }
    }
    /// Создаёт спрайт, загружая его картинку из файла по указанному пути
    /// аттрибуты width и height это ширина и высота спрайта
    pub fn build_sprite(&self, path:&Path, scale: f64) -> Sprite
    {
        let texture = TextureLoader::load_rgba_texture(path, self.display);
        let rect = Rect::from_scaled(texture.dimensions(), scale);
        Sprite::new(rect, texture, 1, 1, 1, 1, false)
    }
    /// Создаёт новый спрайт, передавая ему все необходимые данные.
    pub fn new_sprite(&self, frames_h: u32, frames_v: u32, _cur_frame: u32,
                      layer: u32, _hidden: bool, path:&Path, scale: f64) -> Sprite
    {
        let texture = TextureLoader::load_rgba_texture(path, self.display);
        let rect = Rect::from_scaled(texture.dimensions(), scale);
        Sprite::new(rect, texture, frames_h, frames_v, _cur_frame, layer, _hidden)
    }
    /// Создаёт новый спрайт для фона
    pub fn build_bg(&self, path:&Path) -> Sprite {
        let texture = TextureLoader::load_rgba_texture(path, self.display);
        let rect = Rect::new(Point2::new(0.0, 0.0),
        PhysicalSize::new(self.screen_size.width as f64, self.screen_size.height as f64));
        Sprite::new(rect, texture, 1, 1, 1, 0, false)
    }
    /// Рисует спрайт на указанном фрейме
    pub fn draw(&self, sprite: &Sprite, frame: &mut glium::Frame) {
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
            let rect = sprite.get_rect();
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
            tex: &sprite.texture,
        };
        frame.draw(
                &rect_vertices,
                &rect_indices,
                self.program,
                &uniforms,
                &self.draw_parameters
        ).unwrap();
    }
    /// Задаёт дефолтные параметры рисования спрайтов
    pub fn draw_parameters_default() -> glium::DrawParameters<'static> {
        glium::DrawParameters {
            blend: glium::draw_parameters::Blend
            {
                color: BlendingFunction::Addition
                {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: BlendingFunction::Addition
                {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0)
            },
            .. Default::default()
        }
    }
    /// Возвращает перспективу по умолчанию
    pub fn perspective_default(screen_width: u32, screen_height: u32) -> [[f32; 4]; 4] {
        let matrix: Matrix4<f32> = cgmath::ortho(
            0.0,
            screen_width as f32,
            screen_height as f32,
            0.0,
            -1.0,
            1.0
        );
        Into::<[[f32; 4]; 4]>::into(matrix)
    }
}
