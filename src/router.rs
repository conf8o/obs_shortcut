use anyhow::Result;
use obws::Client;

type ShortCutIndex = usize;
type Process = fn (&Client) -> Result<()>;


pub struct Router {
    pub handlers: Vec<Process>
}

impl Router {
    fn handle(&self, client: &Client, index: ShortCutIndex) -> Result<()>  {
        let handle = &self.handlers[index];
        handle(client)
    }

    pub fn init(handler_pairs: Vec<(ShortCutIndex, fn (&Client) -> Result<()>)>) -> Router {
        let max_index = *handler_pairs.iter().map( |(i, _)| i ).max().unwrap();
        let mut handlers: Vec<Process> = vec![no_process; max_index];
        for (index, handler) in &handler_pairs {
            handlers[*index] = *handler;
        }

        Router { handlers: handlers }
    }
}

pub fn no_process(_ : &Client) -> Result<()> {
    Ok(())
}
