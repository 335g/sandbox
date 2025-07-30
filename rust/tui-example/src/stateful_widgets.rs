use tokio::sync::mpsc;

use crate::event::AppEvent;

#[derive(Debug)]
pub struct StatefulWidgets {
    sender: mpsc::Sender<AppEvent>,
}

impl StatefulWidgets {
    pub fn new(sender: mpsc::Sender<AppEvent>) -> Self {
        Self { sender }
    }
}
