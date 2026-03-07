use iced::{Element, Task};

pub mod styles;

pub trait View<Message> {
    fn view(&self) -> Element<'_, Message>;
    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }
}
