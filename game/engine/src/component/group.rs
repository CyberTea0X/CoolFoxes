use glium::buffer::BufferMode::Default;
use queues::{IsQueue, Queue};
use crate::component::Component;
use crate::component::messages::ComponentMessage;
use crate::graphics::Sprite;
use crate::group::{Group, SomeGroup};
use crate::messaging::traits::Messaging;
use crate::messaging::message::Message;

#[derive(Debug)]
pub struct ComponentsGroup {
    group: Group<Component>,
    input_messages: Queue<Message>,
    output_messages: Queue<Message>
}

impl ComponentsGroup {
    pub fn new() -> Self {
        ComponentsGroup {
            group: Group::new(),
            input_messages: Queue::new(),
            output_messages: Queue::new()
        }
    }
    /// Создаёт группу из вектора элементов
    pub fn from(elements: Vec<Component>) -> Self {
        ComponentsGroup {
            group: Group::from(elements),
            input_messages: Queue::new(),
            output_messages: Queue::new()
        }
    }
    /// Обновляет каждый компонент в группе
    pub fn updated(mut self, dt:u32, mut host: Sprite) -> (ComponentsGroup, Sprite){
        let mut output_messages = Vec::new();
        self.call(|mut c| {
            c = c.updated(dt, &mut host);
            let messages_len = c.output_messages().size();
            for i in 0..messages_len {
                output_messages.push(c.output_messages().remove().unwrap());
            }
            c
        });
        self.call(|mut c| {
            for msg in &output_messages
            {
                c.reveive_msg(msg.clone());
            }
            c.handle_messages();
            c
        });
        return (self, host)
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

impl Messaging for ComponentsGroup {
    type Message = Message;
    fn input_messages(&mut self) -> &mut Queue<Self::Message> {
        &mut self.input_messages
    }
    fn output_messages(&mut self) -> &mut Queue<Self::Message> {
        &mut self.output_messages
    }
    fn handle_message(&mut self, _message: Self::Message) {

    }
}
