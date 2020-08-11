use std::net::TcpListener;
use std::sync::Arc;
use crossbeam_channel::Sender;

use crate::{Request, Cache};

pub fn start(url: String, cache: Arc<Cache>, process_s: Sender<Request>) {
  let listener = TcpListener::bind(url).unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(client) => {

        let request = Request {
          client,
          lease: None,
          headers: None,
          chunk: None
        };

        // TODO
        //
        // can server handle the load?
        // decline client but send back stats
        // about when to request back
        // do I need to process the headers 
        // first before I do this?
        
        process_s.send(request).unwrap();
      },
      Err(err) => {
        println!("{}", err);
      }
    }
  }
}