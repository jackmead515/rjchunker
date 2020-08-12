use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

use crossbeam_utils::thread as cross_thread;
use crossbeam_channel::{unbounded, Sender, Receiver};

pub mod headers;
pub mod io;
pub mod errors;
pub mod process;
pub mod server;
pub mod assembler;

use crate::process::Lease;
use crate::headers::{Headers, HeaderType};

#[derive(Debug)]
pub struct Request {
    client: TcpStream,
    lease: Option<Lease>,
    headers: Option<Headers>,
    chunk: Option<Vec<u8>>
}

impl Request {
    pub fn set_header_type(&mut self, header_type: HeaderType) {
        if let Some(headers) = self.headers.as_mut() {
            headers.set_header_type(header_type);
        }
    }
}

pub struct Cache {
    leases: Mutex<HashMap<String, Lease>>
}

pub fn start_server(url: String) {
    let (process_s, process_r): (Sender<Request>, Receiver<Request>) = unbounded();
    let (assembler_s, assembler_r): (Sender<Request>, Receiver<Request>) = unbounded();
    let cache = Arc::new(Cache {
        leases: Mutex::new(HashMap::new()),
    });

    cross_thread::scope(|scope| {
        let s_cache = cache.clone();
        scope.spawn(move |_| server::start(url, s_cache, process_s));

        let p_cache = cache.clone();
        scope.spawn(move |_| process::start(p_cache, process_r, assembler_s));

        let a_cache = cache.clone();
        scope.spawn(move |_| assembler::start(a_cache, assembler_r));
    }).expect("Failed to create scope");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
