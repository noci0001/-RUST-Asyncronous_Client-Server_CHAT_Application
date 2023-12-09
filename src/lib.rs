use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub mod utils;

//to send different variants of messages
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Client{
  Join{
    chat_name: Arc<String>,
  },
  Post {
    chat_name: Arc<String>,
    message: Arc<String>,
  }
}

pub enum Server {
  Message {
    chat_name: Arc<String>,
    message: Arc<String>,
  },
  Error(String)
}
