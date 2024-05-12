use obws::{Client, Result};
use futures::future::BoxFuture;
pub type ProcessIndex = usize;
pub type Process = fn (&Client) -> BoxFuture<'_, Result<()>>;


pub struct Router {
    processes: Vec<Process>
}

impl Router {
    pub async fn process(&self, client: &Client, index: ProcessIndex) -> Result<()>  {
        let handle = &self.processes[index];
        (handle(client)).await
    }

    pub fn init(handler_pairs: Vec<(ProcessIndex, Process)>) -> Router {
        let max_index = *handler_pairs.iter().map( |(i, _)| i ).max().unwrap();
        let mut processes: Vec<Process> = vec![|client| Box::pin(no_process(client)); max_index+1];
        for (index, process) in &handler_pairs {
            processes[*index] = *process;
        }

        Router { processes }
    }
}

pub async fn no_process(_ : &Client) -> Result<()> {
    Ok(())
}
