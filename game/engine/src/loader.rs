use std::path::Path;
use glium::texture::SrgbTexture2d;

pub struct TextureLoader;

impl TextureLoader {
    /// Загружает текстуру из файла по указанному пути
    pub fn load_rgba_texture(path:&Path, display: &glium::Display) -> SrgbTexture2d {
        let img = image::open(path).unwrap().to_rgba16();
        let img_dim = img.dimensions();
        let img = glium::texture::RawImage2d
        ::from_raw_rgba_reversed(&img.into_raw(), img_dim);
        SrgbTexture2d::new(display, img).unwrap()
    }
}