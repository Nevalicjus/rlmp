use std::time::Duration;

use anyhow::anyhow;
use chrono::NaiveDate;
use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::state::WeekdayData;

const TICK_FPS: f64 = 30.0;

#[derive(Clone, Debug)]
pub enum Event {
    /// Actions happening on a fixed frame rate
    Tick,
    /// Crossterm events, emitted by the terminal
    Crossterm(CrosstermEvent),
    /// Application events
    App(AppEvent)
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    Increment,
    Decrement,
    ChangeCity,
    NextChartType,
    PrevChartType,
    CycleAddtInfo,
    WeatherLoaded { date: NaiveDate, data: WeekdayData },
    WeatherError { date: NaiveDate, err_msg: String },
    Quit
}

#[derive(Debug)]
pub struct EventHandler {
    pub send: mpsc::UnboundedSender<Event>,
    recv: mpsc::UnboundedReceiver<Event>
}

impl EventHandler {
    pub fn new() -> Self {
        let (send, recv) = mpsc::unbounded_channel();
        let actor = EventTask::new(send.clone());
        // spawns the thread that handles events
        tokio::spawn(async { actor.run().await });
        return Self { send, recv };
    }

    /// Receives event from the sender  \
    /// Blocks until an event is received
    ///
    /// # Errors
    /// Returns an error if the sender channel is disconnected.
    /// This can happen if the event thread errs, but in practice shouldn't happen
    /// unless there's a problem with the terminal
    pub async fn next(self: &mut Self) -> anyhow::Result<Event> {
        return self.recv.recv().await.ok_or(anyhow!("Failed to receive event"));
    }

    /// Queue an app event to send to event receiver
    pub fn send(self: &mut Self, app_event: AppEvent) {
        // ignored: While this struct exists, recv can't be dropped, so this show never err
        let _ = self.send.send(Event::App(app_event));
    }
}

/// Thread handling reading crossterm events and emitting ticks
struct EventTask {
    send: mpsc::UnboundedSender<Event>
}

impl EventTask {
    fn new(send: mpsc::UnboundedSender<Event>) -> Self {
        return Self { send };
    }

    async fn run(self) -> anyhow::Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut reader = crossterm::event::EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);
        loop {
            let tick_delay = tick.tick();
            let crossterm_event = reader.next().fuse();
            tokio::select! {
                _ = self.send.closed() => { break; }
                _ = tick_delay => { self.send(Event::Tick); }
                Some(Ok(event)) = crossterm_event => {
                    self.send(Event::Crossterm(event));
                }
            };
        };
        return Ok(());
    }

    fn send(self: &Self, event: Event) {
        // ignored: quitting drops recv, send will always fail then so should not panic
        let _ = self.send.send(event);
    }
}
