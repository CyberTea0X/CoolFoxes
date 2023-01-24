pub struct Group<T> {
    elements: Vec<Option<T>>,
}
impl <T> Group<T> {
    pub fn new() -> Group<T> {
        Group {elements: Vec::new()}
    }
    /// Создаёт группу из вектора элементов
    pub fn from(elements: Vec<T>) -> Group<T> {
        Group {elements: elements.into_iter().map(|s| Some(s)).collect()}
    }
    pub fn reserve(&mut self, additional: usize) {
        self.elements.reserve(additional)
    }
    /// Кладёт элемент в группу, группа забирает себе элемент полностью, никаких ссылок!
    pub fn put(&mut self, mut element: T) {
        self.elements.push(Some(element));
    }
    /// Вызывает closure для каждого элемента в группе.
    /// closure получает элемент и делает с ним что хочет, но возвращает спрайт
    pub fn call<F>(&mut self, mut closure: F)
    where F: FnMut(T) -> T
    {
        let mut element: T;
        for el in &mut self.elements {
            if let None = el {
                continue;
            }
            element = closure(el.take().unwrap());
            el.replace(element);
        }
    }
}