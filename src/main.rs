use iced::{Alignment, button, Button, Column, Element, Length, Row, Sandbox, Settings, Text};


fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0, increment_button: Default::default(), decrement_button: Default::default() }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

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

    fn view(&mut self) -> Element<Message> {
        Column::new().push(
            Text::new("Counter")
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
        )
            .align_items(Alignment::Center).into()
    }
}