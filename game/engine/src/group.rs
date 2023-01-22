struct Group<T> {
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
}