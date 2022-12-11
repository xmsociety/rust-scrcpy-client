use rust_scrcpy_client::ui::constants;
use chrono::Local;

use iced::{Alignment, button, Button, Column, Element, Length, Row, Sandbox, Settings, Text, container};
use iced::alignment::Horizontal;


fn main() -> iced::Result {
    // run forever
    Scrcpy::run(Settings::default())
}

struct Scrcpy {
    value: i32,
    current_time: String,
    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Scrcpy {
    type Message = Message;
    fn new() -> Self {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let t = Local::now().format(fmt).to_string();
        Self { value: 0, current_time: t, increment_button: Default::default(), decrement_button: Default::default() }
    }
    // 标题栏
    fn title(&self) -> String {
        String::from(constants::APP_NAME)
    }

    // 视图刷新
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
    // 布局
    fn view(&mut self) -> Element<Message> {
        Column::new().push(
            Text::new(&self.current_time).horizontal_alignment(Horizontal::Left)
        ).push(
            Row::new().push(
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed).width(Length::Fill),
            ).push(
                Text::new(self.value.to_string()).size(22),
            ).push(
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed).width(Length::Fill),
            )
        ).into()
        // .align_items(Alignment::Center).into()
    }
}