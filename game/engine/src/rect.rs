use glium::backend::Facade;

/// Квадрат, у которого есть left это x, а bottom это y, а также есть ширина и высота
#[derive(Clone, Copy)]
pub struct Rect {
    left: f64,
    bottom: f64,
    width: u32,
    height: u32,
}

impl Rect {
    /// Создаёт квадрат со всеми аттрибутами равными нулю
    pub fn new() -> Rect {
        Rect {left: 0.0, bottom: 0.0, width: 0, height: 0}
    }
    /// Создаёт квадрат с указанными шириной и высотой. x и y устанавливает равными 0
    pub fn from(width: u32, height: u32) -> Rect {
        Rect {left: 0.0, bottom: 0.0, width, height}
    }
    /// Создаёт программу для рисования квадратов из двух шейдеров
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
}
impl Rectangular for Rect {
    fn get_rect(&self) -> &Rect {
        self
    }
    fn get_rect_mut(&mut self) -> &mut Rect {
        self
    }
}

/// Добавляет необходимые любому квадратному объекту методы
pub trait Rectangular {
    /// Получить неизменяемую ссылку на квадрат
    fn get_rect(&self) -> &Rect;
    /// Получить изменяемую ссылку на квадрат
    fn get_rect_mut(&mut self) ->&mut Rect;
    /// Установить Rect объекта позицию left, bottom
    fn with_position <X, Y>(mut self, left: X, bottom: Y) -> Self
    where
        X: Into<f64>,
        Y: Into<f64>,
        Self: Sized
    {
        self.get_rect_mut().left = left.into();
        self.get_rect_mut().bottom = bottom.into();
        return self;
    }
    /// Возвращает левую точку объекта по x
    fn left(&self) -> f64 {
        self.get_rect().left
    }
    /// Возвращает крайнюю нижнюю точку объекта по y
    fn bottom(&self) -> f64 {
        self.get_rect().bottom
    }
    /// Возвращает ширину объекта
    fn width(&self) -> u32 {
        self.get_rect().width
    }
    /// Возвращает высоту объекта
    fn height(&self) -> u32 {
        self.get_rect().height
    }
    /// Возвращает крайнюю правую точку объекта по x
    fn right(&self) -> f64 {
        self.left() + self.width() as f64
    }
    /// Возвращает крайнюю верхнюю точку объекта по y
    fn top(&self) -> f64 {
        self.bottom() + self.height() as f64
    }
    /// Возвращает x объекта
    fn x(&self) -> f64 {self.left()}
    /// Возвращает y объекта
    fn y(&self) -> f64 {self.bottom()}
    /// Двигает объект в указанную позицию.
    /// Для координат, которые вы не собираетесь менять, можно указать значение None
    fn move_ip<T: Into<f64>>(&mut self, x: Option<T>, y: Option<T>) {
        let mut rect = self.get_rect_mut();
        if let Some(x) = x {
            rect.left = x.into();
        };
        if let Some(y) = y {
            rect.bottom = y.into();
        };
    }
    /// Если дословно, то прибавить к позиции объекта x и y
    fn move_by<A: Into<f64>, B: Into<f64>>(&mut self, x: A, y: B) {
        let mut rect = self.get_rect_mut();
        rect.left += x.into();
        rect.bottom += y.into();
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