#![allow(dead_code)]

use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{
        Modifier, Style, Stylize,
        palette::tailwind::{BLUE, GREEN, SLATE},
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let res = App::default().run(terminal);
    ratatui::restore();
    res
}

struct App {
    should_exit: bool,
    todo_list: TodoList,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|f| f.render_widget(&mut self, f.area()))?;
            if let Event::Key(k) = event::read()? {
                self.handle_key(k);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => self.toggle_status(),
            _ => {}
        }
    }

    fn select_none(&mut self) {
        self.todo_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }

    fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items.get_mut(i).unwrap().status.toggle();
        }
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui List Example")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(Style::default().fg(SLATE.c100).bg(BLUE.c800))
            .bg(SLATE.c950);
        let items: Vec<_> = self
            .todo_list
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let c = if i % 2 == 0 { SLATE.c950 } else { SLATE.c900 };
                ListItem::from(item).bg(c)
            })
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.todo_list.state.selected() {
            match self.todo_list.items[i].status {
                Status::Completed => format!("✓ DONE: {}", self.todo_list.items[i].info),
                Status::Todo => format!("☐ TODO: {}", self.todo_list.items[i].info),
            }
        } else {
            "Nothing selected ...".to_string()
        };

        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(Style::new().fg(SLATE.c100).bg(BLUE.c800))
            .bg(SLATE.c950)
            .padding(Padding::horizontal(1));

        Paragraph::new(info)
            .block(block)
            .fg(SLATE.c200)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            todo_list: TodoList::from_iter([
                (
                    Status::Todo,
                    "Rewrite everything with Rust!",
                    "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust",
                ),
                (
                    Status::Completed,
                    "Rewrite all of your tui apps with Ratatui",
                    "Yes, you heard that right. Go and replace your tui with Ratatui.",
                ),
                (
                    Status::Todo,
                    "Pet your cat",
                    "Minnak loves to be pet by you! Don't forget to pet and give some treats!",
                ),
                (
                    Status::Todo,
                    "Walk with your dog",
                    "Max is bored, go walk with him!",
                ),
                (
                    Status::Completed,
                    "Pay the bills",
                    "Pay the train subscription!!!",
                ),
                (
                    Status::Completed,
                    "Refactor list example",
                    "If you see this info that means I completed this task!",
                ),
            ]),
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
    fn from_iter<T: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: T) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, todo, info)| TodoItem::new(status, todo, info))
            .collect();
        let state = ListState::default();

        Self { items, state }
    }
}

struct TodoList {
    items: Vec<TodoItem>,
    state: ListState,
}

struct TodoItem {
    todo: String,
    info: String,
    status: Status,
}

impl TodoItem {
    fn new(status: Status, todo: &'static str, info: &'static str) -> Self {
        Self {
            todo: todo.to_string(),
            info: info.to_string(),
            status,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Todo,
    Completed,
}

impl Status {
    fn toggle(&mut self) {
        match self {
            Status::Todo => *self = Status::Completed,
            Status::Completed => *self = Status::Todo,
        }
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!(" ☐ {}", value.todo), SLATE.c200),
            Status::Completed => Line::styled(format!(" ✓ {}", value.todo), GREEN.c500),
        };
        ListItem::new(line)
    }
}
