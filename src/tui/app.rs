use chrono::{NaiveDate, TimeDelta};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

use crate::{
    cache,
    state::{State, WeekdayState},
    tui::widgets::{ChartType, InfoType},
    utils::{today, start_of_week, end_of_week}
};
use super::event::{AppEvent, Event, EventHandler};

pub async fn run_app(place: String, selected_date: Option<NaiveDate>) -> anyhow::Result<()> {
    let terminal = ratatui::init();
    let res = App::new(place, selected_date).run(terminal).await;
    ratatui::restore();
    return res;
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub state: State,
    pub events: EventHandler,

    pub addt_info_type: InfoType,
    pub today_chart_type: ChartType,
    pub current_date: NaiveDate,
}

impl App {
    pub fn new(place: String, selected_date: Option<NaiveDate>) -> Self {
        return Self {
            running: true,
            state: State::new(place),
            events: EventHandler::new(),

            addt_info_type: InfoType::TempOverSun,
            today_chart_type: ChartType::Temp,
            current_date: if selected_date.is_none() { today() } else { selected_date.unwrap() }
        };
    }

    pub fn decrement_current_date(self: &mut Self) {
        self.current_date = self.current_date - TimeDelta::days(1);
    }

    pub fn increment_current_date(self: &mut Self) {
        if end_of_week(self.current_date) > today() + TimeDelta::days(21) { return; }
        self.current_date = self.current_date + TimeDelta::days(1);
    }

    pub async fn run(mut self: Self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        self.events.send(AppEvent::ChangeCity);
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => {
                        if key_event.kind == crossterm::event::KeyEventKind::Press {
                            self.handle_key_events(key_event)?
                        }
                    },
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => {
                        self.increment_current_date();
                        self.state.update(
                            self.events.send.clone(),
                            start_of_week(self.current_date), end_of_week(self.current_date)
                        ).await?;
                    },
                    AppEvent::Decrement => {
                        self.decrement_current_date();
                        self.state.update(
                            self.events.send.clone(),
                            start_of_week(self.current_date), end_of_week(self.current_date)
                        ).await?;
                    },
                    AppEvent::ChangeCity => {
                        self.state.warm_caches(today()).await?;
                        self.state.update(
                            self.events.send.clone(),
                            start_of_week(self.current_date), end_of_week(self.current_date)
                        ).await?;
                    }
                    AppEvent::NextChartType => {
                        self.today_chart_type = self.today_chart_type.next();
                    },
                    AppEvent::PrevChartType => {
                        self.today_chart_type = self.today_chart_type.prev();
                    },
                    AppEvent::CycleAddtInfo => {
                        self.addt_info_type = self.addt_info_type.next();
                    },
                    AppEvent::WeatherLoaded { date, data } => {
                        self.state.weekday_data.insert(date, WeekdayState::Ready(data));
                    },
                    AppEvent::WeatherError { date, err_msg } => {
                        self.state.weekday_data.insert(date, WeekdayState::Err(err_msg));
                    },
                    AppEvent::Quit => {
                        cache::Cache::global().save_persist()?;
                        self.quit()
                    }
                }
            }
        }
        return Ok(());
    }

    pub fn handle_key_events(self: &mut Self, key_event: KeyEvent) -> anyhow::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            },
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            KeyCode::Tab => self.events.send(AppEvent::CycleAddtInfo),
            KeyCode::Up => self.events.send(AppEvent::NextChartType),
            KeyCode::Down => self.events.send(AppEvent::PrevChartType),
            _ => {}
        };
        return Ok(());
    }

    /// Handles the tick event of the terminal.
    // no fixed rate application logic
    pub fn tick(self: &mut Self) {}

    pub fn quit(self: &mut Self) {
        self.running = false;
    }
}
