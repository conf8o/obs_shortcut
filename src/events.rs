use crate::router::{Process, ProcessIndex};
use crate::processes::*;
use multiinput::*;


#[derive(Clone)]
struct Event(bool);

impl Event {
    pub fn up(&self) -> Event {
        Event(true)
    }

    pub fn down(&self) -> Event {
        Event(false)
    }

    pub fn from_raw_event(raw_event: RawEvent) -> Option<(ProcessIndex, Event)> {
        match raw_event {
            RawEvent::KeyboardEvent(1, key_id, state) => {
                Some((PRESS_W, Event(true)))
            }

            _ => None
        }
    }
}


struct EventTable {
    states: Vec<Option<Event>>
}

impl EventTable {
    pub fn up(&mut self, index: ProcessIndex) {
        if let Some(s) = &self.states[index] {
            self.states[index] = Some(s.up());
        }
    }

    pub fn down(&mut self, index: ProcessIndex) {
        if let Some(s) = &self.states[index] {
            self.states[index] = Some(s.down());
        }
    }

    pub fn init(indice: Vec<ProcessIndex>) -> EventTable {
        let max_index = *indice.iter().max().unwrap();
        let states: Vec<Option<Event>> = vec![None; max_index+1];

        EventTable { states }
    }
}
