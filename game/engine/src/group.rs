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
        for i in (0..self.elements.len()).rev() {
            let el = &mut self.elements[i];
            if let None = el {
                continue;
            }
            element = closure(el.take().unwrap());
            el.replace(element);
        }
    }
}
trait GetElements <T> {
    fn get_elements(&self) -> &Vec<T>;
    fn get_elements_mut(&self) -> &mut Vec<T>;
}

trait SomeGroup<T>:GetElements<T> {

}