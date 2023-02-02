use crate::component::{Component, ComponentsGroup};
use crate::group::SomeGroup;

pub trait Composite {
    fn get_components(&self) -> &Option<ComponentsGroup>;
    fn get_components_mut(&mut self) -> &mut Option<ComponentsGroup>;
    fn with_components(mut self, components: Vec<Component>) -> Self
        where Self:Sized
    {
        let components_gr = self.get_components_mut();

        match components_gr {
            Some(group) => group.append(components),
            None => {components_gr.replace(ComponentsGroup::from(components));},
        }
        return self;
    }
    fn with_component(mut self, component: Component) -> Self
        where Self:Sized
    {
        let components_gr = self.get_components_mut();

        match components_gr {
            Some(group) => group.push(component),
            None => {components_gr.replace(ComponentsGroup::from(vec!(component)));},
        }
        return self;
    }

}