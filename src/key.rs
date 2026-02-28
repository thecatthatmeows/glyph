use crossterm::event;
use parking_lot::Mutex;
use std::{collections::HashSet, sync::Arc, time::Duration};

// for export
pub type KeyCode = event::KeyCode;

pub struct KeyInput {
    current: Arc<Mutex<HashSet<KeyCode>>>,
    previous: Arc<Mutex<HashSet<KeyCode>>>,
}

impl KeyInput {
    pub fn new() -> Self {
        let current = Arc::new(Mutex::new(HashSet::new()));
        let previous = Arc::new(Mutex::new(HashSet::new()));

        let current_clone = Arc::clone(&current);
        let previous_clone = Arc::clone(&previous);

        std::thread::spawn(move || {
            loop {
                Self::poll_key_event(&current_clone, &previous_clone);
            }
        });
        Self { current, previous }
    }

    fn poll_key_event(curr: &Arc<Mutex<HashSet<KeyCode>>>, previous: &Arc<Mutex<HashSet<KeyCode>>>) {
        let mut prev = previous.lock();
        let mut curr = curr.lock();
        *prev = curr.clone();
        while event::poll(Duration::from_millis(0)).unwrap() {
            if let event::Event::Key(event) = event::read().unwrap() {
                match event.kind {
                    event::KeyEventKind::Press | event::KeyEventKind::Repeat => { curr.insert(event.code); }
                    event::KeyEventKind::Release => { curr.remove(&event.code); }
                }
            }
        }
    }

    pub fn is_down(&mut self, key: &KeyCode) -> bool {
        self.current.lock().contains(key)
    }

    pub fn is_pressed(&mut self, key: &KeyCode) -> bool {
        self.current.lock().contains(key) && !self.previous.lock().contains(key)
    }

    pub fn is_released(&mut self, key: &KeyCode) -> bool {
        !self.current.lock().contains(key) && self.previous.lock().contains(key)
    }
}
