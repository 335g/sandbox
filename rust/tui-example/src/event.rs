use ratatui::crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use tokio::{
    sync::mpsc,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AppEvent {
    Quit,
    Tick,
    Resize(u16, u16),
    PressedKey(KeyEvent),
}

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<AppEvent>,
    receiver: mpsc::Receiver<AppEvent>,
    _handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel(1);
        let handler = {
            let sender = sender.clone();
            tokio::spawn(async move {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(timeout).expect("successfully polled for events") {
                        Self::consume_event(&sender).await;
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender
                            .send(AppEvent::Tick)
                            .await
                            .expect("event should have been sent");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            _handler: handler,
        }
    }

    pub fn sender(&self) -> &mpsc::Sender<AppEvent> {
        &self.sender
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        self.receiver.recv().await
    }

    async fn consume_event(sender: &mpsc::Sender<AppEvent>) {
        match event::read() {
            Ok(Event::Key(k)) if k.kind == KeyEventKind::Press => {
                sender
                    .send(AppEvent::PressedKey(k))
                    .await
                    .expect("send pressed key event");
            }
            Ok(Event::Resize(w, h)) => {
                sender
                    .send(AppEvent::Resize(w, h))
                    .await
                    .expect("send resize event");
            }
            _ => {}
        }
    }
}
