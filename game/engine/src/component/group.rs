use crate::component::Component;
use crate::group::{Group, SomeGroup};

impl ComponentsGroup {
    pub fn new() -> Self {
        ComponentsGroup {
            group: Group::new()
        }
    }
    /// Создаёт группу из вектора элементов
    pub fn from(elements: Vec<Component>) -> Self {
        ComponentsGroup {group: Group::from(elements)}
    }
}

impl SomeGroup<Component> for ComponentsGroup {
    fn get_elements(&self) -> &Vec<Option<Component>> {
        self.group.get_elements()
    }
    fn get_elements_mut(&mut self) -> &mut Vec<Option<Component>> {
        self.group.get_elements_mut()
    }
}

#[derive(Debug)]
pub struct ComponentsGroup {
    group: Group<Component>
}
