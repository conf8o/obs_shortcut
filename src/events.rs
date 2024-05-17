#[derive(Clone)]
struct Event(usize, bool);

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
        if raw_event.0 < 0x41 || raw_event.0 > 0x5A {
            return None
        }
        
        Some(Event((raw_event.0 - 0x41) as usize, Event::as_bool(raw_event.1)))
    }

    fn as_bool(s: State) -> bool {
        if s == 0 { true } else { false }
    }
}


pub struct EventTable {
    states: Vec<Option<Event>>
}

impl EventTable {
    fn up(&mut self, index: usize) {
        if let Some(s) = &self.states[index] {
            self.states[index] = Some(s.up());
        }
    }

    fn down(&mut self, index: usize) {
        if let Some(s) = &self.states[index] {
            self.states[index] = Some(s.down());
        }
    }

    pub fn init(indices: &Vec<usize>) -> EventTable {
        let max_index = indices.iter().max().unwrap();
        let mut states: Vec<Option<Event>> = vec![None; max_index+1];

        for index in indices {
            states[*index] = Some(Event(*index, false));
        }

        EventTable { states }
    }

    pub fn handle_from_raw_event(&mut self, raw_event: RawEvent) -> Option<usize> {
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
