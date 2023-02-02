pub mod group;
pub mod traits;
pub mod messages;

use glium::glutin::event::VirtualKeyCode::C;
use queues::Queue;
pub use group::ComponentsGroup;
use crate::component::messages::ComponentMessage;
use crate::graphics::Sprite;
use crate::messaging::message::Message;
use crate::messaging::traits::Messaging;
use crate::misc_traits::update::Updatable;

use crate::physics::components::PhysicsComponent;

#[derive(Debug)]
pub enum Component {
    PhysicsComponent(PhysicsComponent),
}

impl Component {
    pub fn updated(mut self, dt:u32, host: &mut Sprite) -> Component {
        return match self {
            Component::PhysicsComponent(p) => Component::PhysicsComponent(p.updated(dt, host)),
        }
    }
}

impl Messaging for Component {
    type Message = ComponentMessage;
    fn input_messages(&mut self) -> &mut Queue<Self::Message> {
        return match self {
            Component::PhysicsComponent(p) => p.input_messages(),
        }
    }
    fn output_messages(&mut self) -> &mut Queue<Self::Message> {
        return match self {
            Component::PhysicsComponent(p) => p.output_messages(),
        }
    }
    fn handle_messages(&mut self) {
        match self {
            Component::PhysicsComponent(p) => p.handle_messages(),
        }
    }
    fn handle_message(&mut self, _message: Self::Message) {

    }
}
