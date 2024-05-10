type ShortCutIndex = usize;
type Process = fn () -> i32;
pub struct Router {
    pub handlers: Vec<Process>
}

impl Router {
    fn handle(&self, index: ShortCutIndex) -> i32 {
        let handle = &self.handlers[index];
        handle()
    }

    pub fn init(handler_pairs: Vec<(ShortCutIndex, fn () -> i32)>) -> Router {
        let max_index = *handler_pairs.iter().map( |(i, _)| i ).max().unwrap();
        let mut handlers: Vec<Process> = vec![|| 0; max_index];
        for (index, handler) in &handler_pairs {
            handlers[*index] = *handler;
        }

        Router { handlers: handlers}
    }
}
