pub struct Group<T> {
    elements: Vec<T>,
}

impl <T> Group<T> {
    pub fn new() -> Group<T> {
        Group {elements: Vec::new()}
    }
    pub fn from(elements: Vec<T>) -> Group<T> {
        Group {elements}
    }
    pub fn reserve(&mut self, additional: usize) {
        self.elements.reserve(additional)
    }
    pub fn add(&mut self, mut element: T) -> &T {
        self.elements.push(element);
        self.elements.last().unwrap()
    }
    pub fn call<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T)
    {
        for mut element in &mut self.elements {
            f(element)
        }
    }
}