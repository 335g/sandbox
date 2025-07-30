use std::time::Duration;

use color_eyre::Result;
use fake::{
    Fake, Faker,
    faker::{self, internet::ja_jp},
};
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Clone)]
struct Grid {
    cols: usize,
    rows: usize,
    texts: Vec<String>,
}

impl Widget for &Grid {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let col_contraints = (0..self.cols).map(|_| Constraint::Length(9));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_contraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            if let Some(text) = self.texts.get(i) {
                Paragraph::new(text.clone())
                    .block(Block::bordered())
                    .render(cell, buf);
            }
        }
    }
}

#[derive(Debug)]
struct Model {
    state: RunningState,
    grid: Grid,
}

impl Model {
    fn new() -> Self {
        let name = fake::faker::name::ja_jp::Name();
        let name = name.fake::<String>();

        Self {
            state: RunningState::Running,
            grid: Grid {
                cols: 1,
                rows: 1,
                texts: vec![name],
            },
        }
    }
}

impl Model {
    fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::IncreaseRow => {
                self.increase_row();
            }
            Message::IncreaseColumn => {
                self.increase_column();
            }
            Message::DecreaseRow => {
                self.decrease_row();
            }
            Message::DecreaseColumn => {
                self.decrease_column();
            }
            Message::Quit => {
                self.state = RunningState::Done;
            }
        }

        None
    }

    fn increase_row(&mut self) {
        let mut texts = vec![];
        for _ in 0..self.grid.cols {
            let name = fake::faker::name::ja_jp::Name();
            let name = name.fake::<String>();
            texts.push(name);
        }

        self.grid.texts.append(&mut texts);
        self.grid.rows += 1;
    }

    fn decrease_row(&mut self) {
        if self.grid.rows == 1 {
            return;
        }

        for _ in 0..self.grid.cols {
            self.grid.texts.pop();
        }
        self.grid.rows -= 1;
    }

    fn increase_column(&mut self) {
        let mut texts = vec![];
        for _ in 0..self.grid.rows {
            let name = fake::faker::name::ja_jp::Name();
            let name = name.fake::<String>();
            texts.push(name);
        }

        self.grid.texts.append(&mut texts);
        self.grid.cols += 1;
    }

    fn decrease_column(&mut self) {
        if self.grid.cols == 1 {
            return;
        }

        for _ in 0..self.grid.rows {
            self.grid.texts.pop();
        }
        self.grid.cols -= 1;
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Clone)]
enum Message {
    IncreaseRow,
    IncreaseColumn,
    DecreaseRow,
    DecreaseColumn,
    Quit,
}

fn view(model: &Model, frame: &mut Frame) {
    frame.render_widget(&model.grid, frame.area());
}

fn handle_event(_model: &Model) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(k) = event::read()?
        && k.kind == KeyEventKind::Press
    {
        return match k.code {
            KeyCode::Char('j') => Ok(Some(Message::IncreaseRow)),
            KeyCode::Char('k') => Ok(Some(Message::DecreaseRow)),
            KeyCode::Char('h') => Ok(Some(Message::DecreaseColumn)),
            KeyCode::Char('l') => Ok(Some(Message::IncreaseColumn)),
            KeyCode::Char('q') => Ok(Some(Message::Quit)),
            _ => Ok(None),
        };
    }

    Ok(None)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut model = Model::new();

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
