use measurements::Mass;
use queues::Queue;
use crate::component::Component;
use crate::component::messages::ComponentMessage;
use crate::graphics::Sprite;
use crate::messaging::message::Message;
use crate::messaging::traits::Messaging;
use crate::misc_traits::update::Updatable;
use crate::physics::messages::PhysicsMessage;
use crate::rect::Rectangular;

mod physics_comp;

pub use physics_comp::PhysicsComponent;
