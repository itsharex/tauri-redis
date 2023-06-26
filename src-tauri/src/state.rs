use chrono::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::oneshot;

pub struct PubsubItem(pub oneshot::Sender<()>, pub DateTime<Local>);

impl PubsubItem {}

pub struct PubsubManager(pub Mutex<HashMap<String, PubsubItem>>);

impl PubsubManager {
    pub fn new() -> PubsubManager {
        PubsubManager(Mutex::new(HashMap::new()))
    }
    pub fn add(&self, name: String, item: PubsubItem) {
        self.0.lock().unwrap().insert(name, item);
    }
}