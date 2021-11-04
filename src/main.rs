use iced::{button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Text,
};

struct GUI {
    tick_state: TickState,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
}

pub enum TickState {
    Stopped,
    Ticking,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                tick_state: TickState::Stopped,
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        //prepare dulation text
        let duration_text = "00:00:00.00";

        //prepare start/stop text
        let start_stop_text = match self.tick_state {
            TickState::Stopped => Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
                TickState::Ticking => Text::new("start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        };

        //prepare start/stop message on button press
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };

        //init widgets
        let tick_text = Text::new(duration_text).font(FONT).size(60);
        let start_stop_button = Button::new(&mut self.start_stop_button_state, start_stop_text)
            .min_width(80).on_press(start_stop_message);
        let reset_button = Button::new (
            &mut self.reset_button_state,
            Text::new("reset")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT),
        )
        .min_width(80);
        .on_press(Message::Reset);

        //prepare Column
        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings)
}
