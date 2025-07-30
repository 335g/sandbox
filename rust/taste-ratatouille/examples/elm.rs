#![allow(dead_code)]

use std::time::Duration;

use color_eyre::Result;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::Paragraph,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut model = Model::default();

    while model.state != RunningState::Done {
        terminal.draw(|f| view(&model, f))?;

        let mut current_msg = handle_event(&model)?;

        while let Some(msg) = current_msg {
            current_msg = model.update(msg);
        }
    }

    ratatui::restore();
    Ok(())
}

#[derive(Debug, Default)]
struct Model {
    counter: i32,
    state: RunningState,
}

impl Model {
    fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Increment => {
                self.counter += 1;
                if self.counter > 50 {
                    return Some(Message::Reset);
                }
            }
            Message::Decrement => {
                self.counter -= 1;
                if self.counter < -50 {
                    return Some(Message::Reset);
                }
            }
            Message::Reset => self.counter = 0,
            Message::Quit => self.state = RunningState::Done,
        }

        None
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
}

fn view(model: &Model, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("Counter: {}", model.counter)),
        frame.area(),
    );
}

fn handle_event(_: &Model) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(k) = event::read()?
        && k.kind == KeyEventKind::Press
    {
        return Ok(handle_key(k));
    }

    Ok(None)
}

fn handle_key(k: KeyEvent) -> Option<Message> {
    match k.code {
        KeyCode::Char('j') => Some(Message::Increment),
        KeyCode::Char('k') => Some(Message::Decrement),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}
