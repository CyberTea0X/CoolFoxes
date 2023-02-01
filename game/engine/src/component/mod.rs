pub mod group;
pub mod messages;
pub mod traits;

pub use group::ComponentsGroup;
pub use messages::ComponentMessage;

use crate::physics::components::{PhysicsComponent};

#[derive(Debug)]
pub enum Component {
    PhysicsComponent(PhysicsComponent)
}

impl Component {
    pub fn update(&mut self) {

    }
}
