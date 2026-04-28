use ratatui::{
    Frame, layout::{Constraint, Layout, Rect},
    style::Stylize, text::{Line, Text}
};

use super::app::App;
use crate::{
    tui::widgets::{Today, Weekday},
    utils::{date_range, weekday_short, start_of_week, end_of_week, today}
};

impl App {
    pub fn render(self: &mut Self, frame: &mut Frame) {
        let area = frame.area();

        let header = Text::from_iter([
            Line::from(self.state.place.clone().bold()),
            Line::from("<q> Quit | <→> Next Day | <←> Previous Day | <↑> Next Chart | <↓> Prev Chart | <Tab> Cycle addt. info"),
        ]);

        let [header_area, body_area] = Layout::vertical([
            Constraint::Length(header.height() as u16),
            Constraint::Fill(1),
        ])
        .areas(area);

        frame.render_widget(header.centered(), header_area);

        let [weekday_area, today_area] = Layout::vertical([
            Constraint::Length(10),
            Constraint::Fill(1)
        ]).areas(body_area);

        self.render_week_area(frame, weekday_area);
        self.render_today_area(frame, today_area);
    }

    pub fn render_week_area(self: &mut Self, frame: &mut Frame<'_>, area: Rect) {
        let (start, end) = (start_of_week(self.current_date), end_of_week(self.current_date));
        let chunks = Layout::horizontal(vec![Constraint::Fill(1); 7]).split(area);
        for (idd, (date, area)) in date_range(start, end).zip(chunks.into_iter()).enumerate() {
            let widget = Weekday {
                date: date,
                weather: self.state.get_weather_state(date),
                addt_info_type: self.addt_info_type,
                is_selected: date == self.current_date,
                is_today: date == today(),
                weekday: weekday_short(idd)
            };
            frame.render_widget(widget, *area);
        }
    }

    pub fn render_today_area(self: &mut Self, frame: &mut Frame<'_>, area: Rect) {
        let date = self.current_date;
        let today = Today {
            weather: self.state.get_weather_state(date),
            chart_type: self.today_chart_type
        };
        frame.render_widget(today, area);
    }
}
