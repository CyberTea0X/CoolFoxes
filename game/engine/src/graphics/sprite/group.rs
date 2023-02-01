use crate::graphics::Sprite;
use crate::group::{Group, SomeGroup};
use crate::misc_traits::named::Named;

/// Как обычная группа, только для спрайтов и со специальными для них методами
#[derive(Debug)]
pub struct SpriteGroup {
    group: Group<Sprite>
}

impl SpriteGroup {
    pub fn new() -> Self {
        SpriteGroup {
            group: Group::new()
        }
    }
    /// Создаёт группу из вектора элементов
    pub fn from(elements: Vec<Sprite>) -> Self {
        SpriteGroup {group: Group::from(elements)}
    }
    /// Ищет спрайт по имени
    pub fn find(&self, name: &str) -> Option<usize>{
        let elements = self.get_elements();
        for i in 0..self.get_elements().len() {
            let element = if let Some(sprite)= &elements[i] {sprite} else {continue};
            let sprite_name = if let Some(n) = element.get_name() {n} else {continue};
            if sprite_name.as_str() == name {
                return Some(i);
            }
        }
        return None;
    }
    /// Ищет спрайт по имени и возвращает Option<&Sprite>
    pub fn find_get(&self, name: &str) -> Option<&Sprite> {
        let i = self.find(name);
        match i {
            Some(i) => self.get(i),
            None => None,
        }
    }
    /// Ищет спрайт по имени и возвращает Option<&mut Sprite>
    pub fn find_get_mut(&mut self, name: &str) -> Option<&mut Sprite> {
        let i = self.find(name);
        match i {
            Some(i) => self.get_mut(i),
            None => None,
        }
    }
    /// Ищет спрайт по имени и возвращает Option<Sprite>
    pub fn find_take(&mut self, name: &str) -> Option<Sprite> {
        let i = self.find(name);
        match i {
            Some(i) => self.take_el(i),
            None => None,
        }
    }
}

impl SomeGroup<Sprite> for SpriteGroup {
    fn get_elements(&self) -> &Vec<Option<Sprite>> {
        self.group.get_elements()
    }
    fn get_elements_mut(&mut self) -> &mut Vec<Option<Sprite>> {
        self.group.get_elements_mut()
    }
}
