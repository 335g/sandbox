#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::time::Duration;

use color_eyre::Result;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Clear, Paragraph},
};

#[derive(Debug, Clone, Default)]
struct Model {
    state: RunningState,
}

impl Model {
    fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => self.state = RunningState::Done,
            Message::ShowPopup => self.state = RunningState::ShowingPopup,
            Message::ClosePopup => self.state = RunningState::Running,
        }

        None
    }

    fn handle_event(&self) -> Result<Option<Message>> {
        if event::poll(Duration::from_millis(250))?
            && let Event::Key(k) = event::read()?
            && k.kind == KeyEventKind::Press
        {
            return Ok(self.handle_key(k));
        }

        Ok(None)
    }
    fn handle_key(&self, key: KeyEvent) -> Option<Message> {
        match (self.state, key.code) {
            (RunningState::ShowingPopup, KeyCode::Char('q')) => Some(Message::ClosePopup),
            (_, KeyCode::Char('q')) => Some(Message::Quit),
            (_, KeyCode::Char('x')) => Some(Message::ShowPopup),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
enum RunningState {
    #[default]
    Running,
    ShowingPopup,
    Done,
}

fn center_horizontal(area: Rect, width: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);

    area
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);

    area
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);

    area
}

fn render_popup(frame: &mut Frame) {
    let area = center(
        frame.area(),
        Constraint::Percentage(20),
        Constraint::Length(3),
    );
    let popup = Paragraph::new("Popup content").block(Block::bordered().title("Popup"));
    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

fn view(model: &Model, frame: &mut Frame) {
    if model.state == RunningState::ShowingPopup {
        render_popup(frame);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Message {
    Quit,
    ShowPopup,
    ClosePopup,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut model = Model::default();

    while model.state != RunningState::Done {
        terminal.draw(|f| view(&model, f))?;

        let mut current_msg = model.handle_event()?;

        while let Some(msg) = current_msg {
            current_msg = model.update(msg);
        }
    }

    ratatui::restore();
    Ok(())
}
