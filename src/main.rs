use iced::{
button, Button,
Column, Element, Sandbox,
Settings, Text,
};

#[derive(Default)]
struct Counter {
    // The counter value
    value: i32,
    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

impl Sandbox for Counter {

    type Message = Message;

    fn new() -> Counter {
        Counter {
            value: 0,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", "hello Luiz")
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        Column::new()
            .push(
                // The increment button. We tell it to produce an
                // `IncrementPressed` message when pressed
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
                // We show the value of the counter here
                Text::new(self.value.to_string()).size(50),
            )
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
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
}


#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
