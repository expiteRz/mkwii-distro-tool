use iced::{Element, Subscription, Task, Theme};

pub mod styles;

pub trait Parent<Message> {
    fn title(&self) -> String;
    fn view(&self) -> Element<'_, Message>;

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn theme(&self) -> Option<Theme> {
        None
    }
}

pub trait Child<Message> {
    fn view(&self) -> Element<'_, Message>;
    fn update(&mut self, message: Message) {}
}
