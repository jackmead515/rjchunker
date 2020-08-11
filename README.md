# rjchunker

#### A chunkable lazy server and protocol for high throughput low resource platforms

```rust
// basic program structure
let cache = Arc::new(Cache {});
let (process_s, process_r) = unbounded::<TcpStream>::()
let (assembler_s, assembler_r) = unbounded::<Request>::()

process() {
  loop {
    let socket = process_r.recv()
    let request = read(socket)

    match request.headers {
      LEASE() => {
        save_request(request)
        request.chunk = receive(socket)
        assembler_s.send(request)
      }
      CHUNK() => {
        request.chunk = receive(socket)
        assembler_s.send(request)
      }
      TERMINATE() => {
        request.chunk = receive(socket)
        assembler_s.send(request)
      }
      NONE() => {
        end_request()
      }
    }
  }
}

assembler() {
  loop {
    let request = assembler_r.recv()

    match request.headers {
      LEASE() => {
        save_request(request)
        request.chunk = receive(socket)
        assembler_s.send(request)
      }
      CHUNK() => {
        request.chunk = receive(socket)
        assembler_s.send(request)
      }
      TERMINATE() => {
        request.chunk = receive(socket)
        assembler_s.send(request)
      }
    }
  }
}

scope (
  spawn (process())
  spawn (assembler())
)
```