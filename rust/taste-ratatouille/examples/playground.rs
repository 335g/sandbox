#![allow(dead_code)]
#![allow(unused_variables)]

use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    ratatui::restore();
    Ok(())
}

struct App {
    should_exit: bool,
}

impl App {
    fn new() -> Self {
        Self { should_exit: false }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {}

    fn handle_events(&mut self) -> Result<()> {
        Ok(())
    }
}
