use cgmath::Point2;
use glium::glutin::dpi::{PhysicalSize};
/// Квадрат, позиция которого это его левый нижний угол, а размер это его ширина и высота
#[derive(Clone, Copy)]
pub struct Rect {
    position: Point2<f64>,
    size: PhysicalSize<f64>
}
impl Rect {
    pub fn new(position: Point2<f64>, size: PhysicalSize<f64>) -> Self {
        Rect {position, size}
    }
    pub fn from_scaled<T: Into<f64>>(dimensions: (T, T), scale: f64) -> Self {
        let (width, height) = dimensions;
        Rect::new(Point2::new(0.0, 0.0),
                  PhysicalSize::new(width.into() * scale, height.into() * scale))
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

impl Default for Rect {
    fn default() -> Self {
        Rect::new(Point2::new(0.0, 0.0), PhysicalSize::new(0.0, 0.0))
    }
}

/// Добавляет необходимые любому квадратному объекту методы
pub trait Rectangular {
    /// Получить неизменяемую ссылку на квадрат
    fn get_rect(&self) -> &Rect;
    /// Получить изменяемую ссылку на квадрат
    fn get_rect_mut(&mut self) ->&mut Rect;
    /// Установить Rect объекта позицию left, bottom
    fn with_position <X, Y>(mut self, x: X, y: Y) -> Self
    where
        X: Into<f64>,
        Y: Into<f64>,
        Self: Sized
    {
        self.get_rect_mut().position.x = x.into();
        self.get_rect_mut().position.y = y.into();
        return self;
    }
    /// Возвращает x объекта
    fn x(&self) -> f64 {self.get_rect().position.x}
    /// Возвращает y объекта
    fn y(&self) -> f64 {self.get_rect().position.y}
    /// Возвращает левую точку объекта по x
    fn left(&self) -> f64 {
        self.x()
    }
    /// Возвращает крайнюю нижнюю точку объекта по y
    fn bottom(&self) -> f64 {
        self.y()
    }
    /// Возвращает ширину объекта
    fn width(&self) -> f64 {
        self.get_rect().size.width
    }
    /// Возвращает высоту объекта
    fn height(&self) -> f64 {
        self.get_rect().size.height
    }
    /// Возвращает крайнюю правую точку объекта по x
    fn right(&self) -> f64 {
        self.x() + self.width()
    }
    /// Возвращает крайнюю верхнюю точку объекта по y
    fn top(&self) -> f64 {
        self.y() + self.height()
    }
    /// Двигает объект в указанную позицию.
    /// Для координат, которые вы не собираетесь менять, можно указать значение None
    fn move_ip<T: Into<f64>>(&mut self, x: Option<T>, y: Option<T>) {
        let mut rect = self.get_rect_mut();
        if let Some(x) = x {
            rect.position.x = x.into();
        };
        if let Some(y) = y {
            rect.position.y = y.into();
        };
    }
    /// Если дословно, то прибавить к позиции объекта x и y
    fn move_by<A: Into<f64>, B: Into<f64>>(&mut self, x: A, y: B) {
        let mut rect = self.get_rect_mut();
        rect.position.x += x.into();
        rect.position.y += y.into();
    }
}