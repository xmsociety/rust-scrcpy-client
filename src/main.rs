use std::{thread, time};
use rust_scrcpy_client::ui::constants;
use chrono::Local;

use iced::theme::{self, Theme};
use iced::widget::{button, column, container, row, text, horizontal_space, checkbox};

use iced::{alignment, Alignment, executor, Element, keyboard, Length, Sandbox, Settings, Subscription, window, Application, Command};
use iced::alignment::{Horizontal, Vertical};
use iced::application::StyleSheet;

use iced_native::{event, subscription, Event, Renderer};

fn main() -> iced::Result {
    // run forever
    Scrcpy::run(Settings {
        window: window::Settings {
            size: (constants::WIDTH as u32, constants::HEIGHT as u32),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}


struct Scrcpy {
    current_time: String,
    check_all: bool

}

// event signal
#[derive(Debug, Clone, Copy)]
enum Message {
    UpdateTime,
    UpdateTable,
    Tick(time::Instant),
    StartALl,
    StopAll,
    CheckAll(bool),
    Run,
    Edit,
    Show,
}



impl Application for Scrcpy {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Scrcpy {
                current_time: "".to_string(),
                check_all: false,
            },
            Command::none(),
        )
    }

    // 标题栏
    fn title(&self) -> String {
        String::from(constants::APP_NAME)
    }

    // 视图刷新
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UpdateTime => {
                let fmt = "%Y-%m-%d %H:%M:%S";
                self.current_time = Local::now().format(fmt).to_string();
            }
            Message::UpdateTable => {

            }
            Message::Tick(now) => {

            }
            _ => {

            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message>  {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let current_time = text( Local::now().format(fmt).to_string())
            .size(20).horizontal_alignment(Horizontal::Left);

        let table_headers = row![text("id"), text("check"), text("SerialNum"), text("NickName"), text("RunMode"), text("Run"), text("Operate"), text("Other")].spacing(50).align_items(Alignment::Start).width(Length::Fill);

        let check_box = checkbox(constants::CHECK_ALL, self.check_all, Message::CheckAll);

        let all_start_btn = button(constants::ALL_START)
            .style(theme::Button::Destructive)
            .on_press(Message::StartALl);

        let all_stop_btn = button(constants::ALL_STOP)
            .style(theme::Button::Destructive)
            .on_press(Message::StopAll);


        let controls = row![check_box, all_start_btn, all_stop_btn].spacing(20).align_items(Alignment::End).height(Length::Fill);

        let content = column![current_time, table_headers, controls]
            .spacing(20);
        // let content = container(
        //     column![
        //         row![
        //             text("Top Left"),
        //             horizontal_space(Length::Fill),
        //             text("Top Right")
        //         ]
        //         .align_items(Alignment::Start)
        //         .height(Length::Fill),
        //         container(
        //             button(text("Show Modal")).on_press(Message::StopAll)
        //         )
        //         .center_x()
        //         .center_y()
        //         .width(Length::Fill)
        //         .height(Length::Fill),
        //         row![
        //             text("Bottom Left"),
        //             horizontal_space(Length::Fill),
        //             text("Bottom Right")
        //         ]
        //         .align_items(Alignment::End)
        //         .height(Length::Fill),
        //     ]
        //         .height(Length::Fill),
        // )
        //     .padding(10)
        //     .width(Length::Fill)
        //     .height(Length::Fill);
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_x(Horizontal::Left)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

}