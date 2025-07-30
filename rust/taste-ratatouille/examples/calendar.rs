use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style, Stylize as _},
    widgets::calendar::{CalendarEventStore, Monthly},
};
use time::{Date, Month, OffsetDateTime, Weekday};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let res = run(terminal);
    ratatui::restore();
    res
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if let Event::Key(k) = event::read()?
            && k.code == KeyCode::Char('q')
            && k.kind == KeyEventKind::Press
        {
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    let area = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let year = 2025;
    let month = Month::July;
    let day = 1;
    let d = Date::from_calendar_date(year, month, day).unwrap();
    let list = make_dates(d.year(), d.month());

    let monthly = Monthly::new(d, list)
        .show_month_header(Style::default().bold())
        .show_weekdays_header(Style::default().italic())
        .show_surrounding(Style::default());
    frame.render_widget(monthly, area);
}

fn make_dates(year: i32, month: Month) -> CalendarEventStore {
    let mut list = CalendarEventStore::today(Style::default());
    let holiday_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::UNDERLINED);

    let first_date = Date::from_calendar_date(year, month, 1).unwrap();

    let mut d = first_date;
    loop {
        if d.weekday() == Weekday::Sunday {
            list.add(d, holiday_style);
        }

        d = d.next_day().unwrap();
        if d.month() != first_date.month() {
            break;
        }
    }

    // list.add(
    //     Date::from_calendar_date(current_year, Month::January, 1).unwrap(),
    //     holiday_style,
    // );
    // list.add(
    //     Date::from_calendar_date(current_year + 1, Month::January, 1).unwrap(),
    //     holiday_style,
    // );
    // list.add(
    //     Date::from_calendar_date(current_year, Month::February, 1).unwrap(),
    //     holiday_style,
    // );

    list
}
