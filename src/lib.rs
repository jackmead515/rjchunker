use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::TcpStream;

use crossbeam_channel::{unbounded, Sender, Receiver};

pub mod headers;
pub mod io;
pub mod errors;
pub mod pipe;

pub struct Request {
    headers: headers::Headers
}

pub struct Cache {
    leases: Mutex<HashMap<String, pipe::Lease>>
}

pub struct AssemblerChannels {
    sender: Sender<Request>,
    receiver: Receiver<Request>
}

pub struct PipeChannels {
    sender: Sender<TcpStream>,
    receiver: Receiver<TcpStream>
}

pub struct Server {
    pipe_channels: PipeChannels,
    assembler_channels: AssemblerChannels,
    cache: Arc<Cache>
}

impl Server {
    pub fn new() -> Self {
        let (pipe_s, pipe_r) = unbounded();
        let (assembler_s, assembler_r) = unbounded();

        return Server {
            pipe_channels: PipeChannels {
                sender: pipe_s,
                receiver: pipe_r
            },
            assembler_channels: AssemblerChannels {
                sender: assembler_s,
                receiver: assembler_r
            },
            cache: Arc::new(Cache {
                leases: Mutex::new(HashMap::new()),
            })
        }
    }


}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
