use std::fmt::Debug;
use queues::{IsQueue, Queue};

pub trait Messaging {
    type Message: Clone + Copy + Debug;
    /// Возвращает изменяемую ссылку на входящие сообщения
    fn input_messages(&mut self) -> &mut Queue<Self::Message>;
    /// Возвращает изменяемую ссылку на исходящие сообщения
    fn output_messages(&mut self) -> &mut Queue<Self::Message>;
    /// Получает сообщение
    fn reveive_msg(&mut self, message:Self::Message) {
        self.input_messages().add(message).unwrap();
    }
    /// Отправляет сообщение
    fn send_msg(&mut self, message:Self::Message) {
        self.output_messages().add(message).unwrap();
    }
    /// Обрабатывает все входящие сообщения
    fn handle_messages(&mut self) {
        let msg_len = self.input_messages().size();
        for i in 0..msg_len {
            let oldest_message = self.input_messages().remove().unwrap();
            println!("{:?}", oldest_message);
            self.handle_message(oldest_message);
        }
    }
    /// Обрабатывает входящее сообщение
    fn handle_message(&mut self, message: Self::Message);
}