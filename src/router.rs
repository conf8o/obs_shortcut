//! インデックスと動作を紐づけるテーブル。ルーターと名付けてる。
//! 具体的なProcessはprocessesに定義する。

use obws::{Client, Result};
use futures::future::BoxFuture;
pub type Process = fn (&Client) -> BoxFuture<'_, Result<()>>;


pub struct Router {
    processes: Vec<Process>
}

impl Router {
    pub async fn process(&self, client: &Client, index: usize) -> Result<()>  {
        let handle = &self.processes[index];
        handle(client).await
    }

    pub fn init(process_pairs: &Vec<(usize, Process)>) -> Router {
        let max_index = *process_pairs.iter().map( |(i, _)| i ).max().unwrap();
        let mut processes: Vec<Process> = vec![|client| Box::pin(no_process(client)); max_index+1];
        for (index, process) in process_pairs {
            processes[*index] = *process;
        }

        Router { processes }
    }
}

pub async fn no_process(_ : &Client) -> Result<()> {
    Ok(())
}
