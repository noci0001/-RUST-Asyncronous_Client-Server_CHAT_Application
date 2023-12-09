use async_std::prelude::*;

//Processes of converting data between its in-memory representation and byte string
use serde::de::DeserializeOwned;
//Processes of converting data structured memory from its in-memory representation and 
// to and format that can be transmitted or stored.
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

//implementing custom error types
// CHATERROR => box that contains a dynamic trait object that implements the Error trait
//                  and it is safe to send and share between threads
// 
                    //Box -> smart pointer, allocating on the heap, avoids copying
                            // Error -> Trait
                                    // safe to send and share
                                                  // lifetime long as the life of the program
pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;


pub async fn send_json<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
  O: async_std::io::Write + Unpin,
  P: Serialize,
{
  let mut json = serde_json::to_string(&packet)?;
  json.push('\n');

  leaving.write_all(json.as_bytes()).await?;
  Ok(())
}

pub fn receive<I, T>(incoming: I) -> impl Stream<Item = ChatResult<T>>
where
  I: async_std::io::BufRead + Unpin,
  T: DeserializeOwned,
  {
    incoming.lines().map(|line| -> ChatResult<T>)({
      let li = line?;
      let msg = serde_json::from_str::<T>(&li)?;
      Ok(msg)
    })
  }
