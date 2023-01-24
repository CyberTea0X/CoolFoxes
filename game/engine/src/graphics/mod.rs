pub mod gui;
pub mod sprite;

pub use self::sprite::Sprite;
use glium::implement_vertex;

#[derive(Clone, Copy)]
/// Вершина которая описывается двумя значениями x и y
pub struct Vertex {
    // The fields in Vertex are usually there
    // to be passed into the shader file.
    pub position: [f64; 2],
}

// This line implements the Vertex using a macro inside glium.
// Don't forget to include all of the fields as parameters otherwise
// glium won't pass those into the shader.
implement_vertex!(Vertex, position);
