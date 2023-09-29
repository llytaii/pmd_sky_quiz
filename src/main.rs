use iced::widget::{column, container, text, text_input};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};


pub fn main() -> iced::Result {
    QuizSolver::run(Settings::default());
}

#[derive(Default)]
struct QuizSolver {
    input: String,
    output: Vec<String>,
    data: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

impl Sandbox for QuizSolver {
    type Message = Message;

    fn new() -> Self {
        QuizSolver::default()
    }

    fn title(&self) -> String {
        String::from("Pokemon Mystery Dungeon Sky - Quiz Helper")
    }

    fn fuzzy_find(input: &str, list: Vec<&str>) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();

        *outer: for element in list.iter() {
            for character in input.chars() {
            }
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DataChanged(mut input) => {
                self.output = fuzzy_find(&input);
                self.input = input; 
            }
        }
    }
}

