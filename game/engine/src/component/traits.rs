use crate::component::{Component, ComponentsGroup};
use crate::group::SomeGroup;

pub trait Composite {
    fn get_components(&self) -> &ComponentsGroup;
    fn get_components_mut(&mut self) -> &mut ComponentsGroup;
    fn with_components(mut self, components: Vec<Component>) -> Self
        where Self:Sized
    {
        self.get_components_mut().append(components);
        return self;
    }
}