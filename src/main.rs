use adbutils;
use chrono::Local;
use rust_scrcpy_client::ui::constants;
use std::{thread, time};

use iced;
use iced::theme::{self, Theme};
use iced::widget::{button, checkbox, column, container, row, text};

use iced::alignment::{Horizontal, Vertical};
use iced::application::StyleSheet;
use iced::{alignment, executor, keyboard, window, Alignment, Application, Command, Element, Length, Sandbox, Settings, Subscription, Padding};

use iced_native::{event, subscription, Event, Renderer, row, column};

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
#[derive(Debug)]
struct Device {
    device: adbutils::device::AdbDevice,
}

enum DeviceMessage {
    Select,
    Run,
    Edit,
    EditTing,
    Show,
    Hide,
}

impl Device {
    fn new(description: String) -> Self {
        todo!()
    }

    fn update(&mut self, message: DeviceMessage) {}

    fn view(&self, i: usize) -> Element<DeviceMessage> {
        todo!()
    }
}

#[derive(Debug)]
struct Scrcpy {
    current_time: String,
    check_all: bool,
    start_all: bool,
    stop_all: bool,
    devices: Vec<Device>,
}

// event signal
#[derive(Debug, Clone)]
enum Message {
    UpdateTime,
    UpdateTable,
    Tick(time::Instant),
    StartALl,
    StopAll,
    CheckAll(bool),
}

fn list_devices() -> Vec<Device> {
    let adb = adbutils::client::AdbClient::new(
        String::from("localhost"),
        5037,
        time::Duration::new(10, 0),
    );
    // let client = adb._connect();
    println!("adb version: {:?}", adb.server_version());
    let mut devices = Vec::new();
    for device in adb.devices_list() {
        devices.push(Device { device })
    }
    return devices;
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
                start_all: false,
                stop_all: false,
                devices: list_devices(),
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
            Message::UpdateTable => {}
            Message::Tick(now) => {}
            Message::CheckAll(flag) => {
                self.check_all = flag;
                // TODO
                println!("check all: {}", flag);
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let current_time = text(Local::now().format(fmt).to_string())
            .size(20)
            .horizontal_alignment(Horizontal::Left);

        let table_headers = row![
            text("id"),
            text("check"),
            text("SerialNum"),
            text("NickName"),
            text("RunMode"),
            text("Run"),
            text("Operate"),
            text("Other")
        ]
        .spacing(50)
        .align_items(Alignment::Start)
        .width(Length::Fill);

        let check_box = checkbox(constants::CHECK_ALL, self.check_all, Message::CheckAll);

        let all_start_btn = button(constants::ALL_START)
            .style(theme::Button::Primary)
            .on_press(Message::StartALl);

        let all_stop_btn = button(constants::ALL_STOP)
            .style(theme::Button::Primary)
            .on_press(Message::StopAll);

        let controls = row![check_box, all_start_btn, all_stop_btn]
            .spacing(20)
            .align_items(Alignment::End)
            .height(Length::Fill);

        let content = column![current_time, table_headers, controls].spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_x(Horizontal::Left)
            .padding(Padding{top: 10, right: 0, bottom: 20, left: 10 })
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::new(1, 0)).map(|_| Message::UpdateTime)
    }
}
