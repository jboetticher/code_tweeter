use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    CodeTweeter::run(Settings::default())
}

struct CodeTweeter {
    code: String,
}

#[derive(Debug, Clone, Copy)]
enum CodeTweeterMessage {
    CodeChanged,
    SubmitPressed
}

impl Sandbox for CodeTweeter {
    type Message = CodeTweeterMessage;

    // zarbobo
    fn new() -> Self {
        Self { code: "".to_string() }
    }

    fn title(&self) -> String {
        String::from("code tweeter !!!")
    }

    // state updates based on message woah
    fn update(&mut self, message: CodeTweeterMessage) {
        match message {
            CodeTweeterMessage::CodeChanged => {
                println!("the code changed! :O");
            }
            CodeTweeterMessage::SubmitPressed => {
                println!("stop pressing me uwu");
            }
        }
    }

    fn view(&self) -> Element<CodeTweeterMessage> {
        column![
            button("Change Code").on_press(CodeTweeterMessage::CodeChanged),
            // text(self.value).size(50),
            text("amongus").size(50),
            button("Submit").on_press(CodeTweeterMessage::SubmitPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}