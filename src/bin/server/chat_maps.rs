use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::chats::Chats;

// the ChatTracker struct has a Mutex HashMap ArcString and ArcChat field
//     A HashMap with key an Atomic Reference Counter and a value of Arc Chats
pub struct ChatTracker(Mutex <HashMap <Arc<String>, Arc<Chats>> >);

impl ChatTracker {
  pub fn new() -> ChatTracker {
    ChatTracker(Mutex::new(HasMap::new()))
  }

  pub fn find(&self, name: &String) -> Option<Arc<Chats>>{
    self.0.lock().unwrap().get(name).cloned()
  }

  pub fn find_or_new(&self, name: Arc<String>) -> Arc<Chats> {
    self.0.lock().unwrap().entry(name.clone()).or_insert_with(|| Arc::new(Chats::new(name))).clone()
  }
}
