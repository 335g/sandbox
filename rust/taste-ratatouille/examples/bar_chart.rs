#![allow(dead_code)]

use color_eyre::Result;
use rand::Rng;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize as _},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

struct App {
    should_exit: bool,
    temperatures: Vec<u8>,
}

impl App {
    fn new() -> Self {
        let mut rng = rand::rng();
        let temperatures = (0..16).map(|_| rng.random_range(50..90)).collect();
        Self {
            should_exit: false,
            temperatures,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(k) = event::read()?
            && k.kind == KeyEventKind::Press
            && k.code == KeyCode::Char('q')
        {
            self.should_exit = true;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [title_rect, rect1, rect2] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .spacing(1)
        .areas(frame.area());

        frame.render_widget("Barchart".bold().into_centered_line(), title_rect);
        frame.render_widget(vertical_barchart(&self.temperatures), rect1);
        frame.render_widget(horizontal_barchart(&self.temperatures), rect2);
    }
}

fn vertical_barchart(temperatures: &[u8]) -> BarChart {
    let bars = temperatures
        .iter()
        .enumerate()
        .map(|(hour, value)| {
            Bar::default()
                .value(u64::from(*value))
                .label(Line::from(format!("{hour:>02}:00")))
                .text_value(format!("{value:>3}"))
                .style(temperature_style(*value))
                .value_style(temperature_style(*value).reversed())
        })
        .collect::<Vec<_>>();
    let title = Line::from("Weather (Vertical)").centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::new().title(title))
        .bar_width(6)
}

fn horizontal_barchart(temperatures: &[u8]) -> BarChart {
    let bars = temperatures
        .iter()
        .enumerate()
        .map(|(hour, value)| {
            Bar::default()
                .value(u64::from(*value))
                .label(Line::from(format!("{hour:>02}:00")))
                .text_value(format!("{value:>3}"))
                .style(temperature_style(*value))
                .value_style(temperature_style(*value).reversed())
        })
        .collect::<Vec<_>>();
    let title = Line::from("Weather (Horizontal)").centered();
    BarChart::default()
        .block(Block::new().title(title))
        .data(BarGroup::default().bars(&bars))
        .bar_width(1)
        .bar_gap(0)
        .direction(Direction::Horizontal)
}

fn temperature_style(value: u8) -> Style {
    let g = 255.0 * (1.0 - f64::from(value - 50) / 40.0);
    let c = Color::Rgb(255, g as u8, 0);
    Style::new().fg(c)
}
