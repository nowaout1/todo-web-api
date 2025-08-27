use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::{config::Config, todo_proto::todo_client::TodoClient};

pub mod todo_proto {
    tonic::include_proto!("todo.v1");
}

pub mod config;
pub mod todo;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: Arc<Mutex<TodoClient<Channel>>>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, tonic::transport::Error> {
        let todo_channel = Channel::from_static(config.todo_addr()).connect().await?;
        let todo_service = TodoClient::new(todo_channel);

        Ok(Self {
            todo_service: Arc::new(Mutex::new(todo_service)),
        })
    }
}
