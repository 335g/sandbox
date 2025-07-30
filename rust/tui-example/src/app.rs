use tokio::{sync::mpsc, time::Instant};

use crate::event::AppEvent;

#[derive(Debug)]
pub struct App {
    running: bool,
    sender: mpsc::Sender<AppEvent>,
    last_tick: Instant,
}
