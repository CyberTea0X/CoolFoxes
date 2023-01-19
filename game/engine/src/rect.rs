use glium::backend::Facade;

pub struct Rect {
    left: i32,
    bottom: i32,
    width: u32,
    height: u32,
}

impl Rect {
    pub fn new() -> Rect {
        Rect {left: 0, bottom: 0, width: 0, height: 0}
    }
    pub fn from(width: u32, height: u32) -> Rect {
        Rect {left: 0, bottom: 0, width, height}
    }
    pub fn with_position(mut self, left: i32, bottom: i32) -> Rect {
        self.left = left;
        self.bottom = bottom;
        return self;
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn left(&self) -> i32 {
        self.left
    }
    pub fn right(&self) -> i32 {
        self.left + self.width as i32
    }
    pub fn bottom(&self) -> i32 {
        self.bottom
    }
    pub fn top(&self) -> i32 {
        self.bottom + self.height as i32
    }
    /// Create a program from the two shaders.
    /// A "program" is just a bunch of shaders so you can have multiple programs
    /// for drawing different things.
    pub fn drawing_program<F>(display: &F) -> glium::Program
    where F: Facade
    {
        glium::Program::from_source(
        display,
        VERTEX_SHADER,
        FRAGMENT_SHADER,
        None
    ).unwrap()
    }
    pub fn move_ip(&mut self, x: i32, y: i32) {
        self.left = x;
        self.bottom = y;
    }
}

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