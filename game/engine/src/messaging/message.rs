use crate::component::messages::ComponentMessage;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ComponentMessage(ComponentMessage)
}