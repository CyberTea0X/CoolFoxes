use crate::physics::messages::PhysicsMessage;

#[derive(Debug, Clone, Copy)]
pub enum ComponentMessage {
    PhysicsMessage(PhysicsMessage)
}