/// Группа для чего угодно
/// Группы нужны для того, чтобы вызывать одну и ту же функцию для всех элементов
/// А также эффективно хранить различные элементы, работать с ними
#[derive(Debug)]
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
}

impl <T> SomeGroup<T> for Group<T> {
    fn get_elements(&self) -> &Vec<Option<T>> {
        &self.elements
    }
    fn get_elements_mut(&mut self) -> &mut Vec<Option<T>> {
        &mut self.elements
    }
}

pub trait SomeGroup<T> {
    /// Возвращает ссылку на вектор элементов. Не рекомендуется работать с ними напрямую,
    /// если только вы не делаете на основе группы новую структуру.
    fn get_elements(&self) -> &Vec<Option<T>>;
    /// Возвращает изменяемую ссылку на вектор элементов. Не рекомендуется работать с ними напрямую,
    /// если только вы не делаете на основе группы новую структуру.
    fn get_elements_mut(&mut self) -> &mut Vec<Option<T>>;
    /// Резервирует некоторый размер для внутреннего вектора. Полезно для оптимизации.
    fn reserve(&mut self, additional: usize) {
        self.get_elements_mut().reserve(additional)
    }
    /// Кладёт элемент в группу, группа забирает себе элемент полностью, никаких ссылок!
    fn put(&mut self, element: T) {
        self.get_elements_mut().push(Some(element));
    }
    /// Вызывает closure для каждого элемента в группе.
    /// closure получает элемент и делает с ним что хочет, но возвращает спрайт
    fn call<F>(&mut self, mut closure: F)
    where F: FnMut(T) -> T
    {
        let mut element: T;
        for i in (0..self.get_elements().len()).rev() {
            let el = &mut self.get_elements_mut()[i];
            if let None = el {
                continue;
            }
            element = closure(el.take().unwrap());
            el.replace(element);
        }
    }
    /// Забирает элемент из группы и возвращает его
    fn take_el(&mut self, i: usize) -> Option<T> {
        assert!(i < self.get_elements().len(), "Элемента с индексом {i} не существует");
        let i_last = self.get_elements().len() -1;
        self.get_elements_mut().swap(i, i_last);
        self.get_elements_mut().pop().unwrap()
    }
    /// Возвращает неизменяемую ссылку на элемент под указанным индексом
    fn get(&self, i: usize) -> Option<&T> {
        if let Some(opt) = self.get_elements().get(i) {
            if let Some(t) = opt {
                return Some(t);
            }
        }
        return None;
    }
    /// Возвращает изменяемую ссылку на элемент под указанным индексом
    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if let Some(opt) = self.get_elements_mut().get_mut(i) {
            if let Some(t) = opt {
                return Some(t);
            }
        }
        return None;
    }
}