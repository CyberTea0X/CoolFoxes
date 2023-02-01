use crate::physics::components::PhysicsMessage;

pub enum ComponentMessage {
    PhysicsMessage(PhysicsMessage),
}
