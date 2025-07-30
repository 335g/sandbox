use ratatui::{Frame, layout::Size};

use crate::event::{AppEvent, EventHandler};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

#[derive(Debug)]
pub struct Tui {
    terminal: CrosstermTerminal,
    events: EventHandler,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn draw<F>(&mut self, render_ui: F) -> std::io::Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(render_ui)?;
        Ok(())
    }

    pub fn size(&self) -> Size {
        self.terminal.size().unwrap()
    }

    pub async fn receive_events<F>(&mut self, mut f: F)
    where
        F: FnMut(AppEvent),
    {
        while let Some(event) = self.events.next().await {
            f(event)
        }
    }
}
