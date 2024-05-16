use crate::router::ProcessIndex;
use crate::processes::*;


#[derive(Clone)]
struct Event(ProcessIndex, bool);

pub type Key = u16;
pub type State = u16;
pub type RawEvent = (Key, State);

impl Event {
    pub fn up(&self) -> Event {
        Event(self.0, true)
    }

    pub fn down(&self) -> Event {
        Event(self.0, false)
    }

    pub fn from_raw_event(raw_event: RawEvent) -> Option<Event> {
        match raw_event {
            // B
            (0x42, state) => Some(Event(B, Event::as_bool(state))),
            
            // W
            (0x57, state) => Some(Event(W, Event::as_bool(state))),            

            _ => None
        }
    }

    fn as_bool(s: State) -> bool {
        if s == 0 { true } else { false }
    }
}


pub struct EventTable {
    states: Vec<Option<Event>>
}

impl EventTable {
    fn up(&mut self, index: ProcessIndex) {
        if let Some(s) = &self.states[index] {
            self.states[index] = Some(s.up());
        }
    }

    fn down(&mut self, index: ProcessIndex) {
        if let Some(s) = &self.states[index] {
            self.states[index] = Some(s.down());
        }
    }

    pub fn init(indices: &Vec<ProcessIndex>) -> EventTable {
        let max_index = indices.iter().max().unwrap();
        let mut states: Vec<Option<Event>> = vec![None; max_index+1];

        for index in indices {
            states[*index] = Some(Event(*index, false));
        }

        EventTable { states }
    }

    pub fn handle_from_raw_event(&mut self, raw_event: RawEvent) -> Option<ProcessIndex> {
        let Event(index_input, is_up_input) = Event::from_raw_event(raw_event)?;
        if let Some(Event(_, is_up_table)) = self.states[index_input] {
            // tableのeventがdownで、inputのeventがupなら、processを走らせたいので、process_indexを返す
            match (is_up_input, is_up_table) {
                (true, true) => None,
                (true, false) => {
                    self.up(index_input);
                    Some(index_input)
                }
                (false, true) => {
                    self.down(index_input);
                    None
                }
                (false, false) => None
            }
        } else {
            None
        }
    }
}
