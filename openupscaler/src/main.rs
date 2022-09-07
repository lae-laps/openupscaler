// main.rs

use iced::{executor, Application, Command, Element, Settings, Text};

pub fn main() -> iced::Result {
    MainGUI::run(Settings::default())
}

struct MainGUI {
    value: i32,
}

impl Application for MainGUI {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (MainGUI, Command<Self::Message>) {
        (MainGUI, Command::none())
    }

    fn title(&self) -> String {
        String::from("OpenUpscaler")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("MainGUI, world!").into()
    }
}

