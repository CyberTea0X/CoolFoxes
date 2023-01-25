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