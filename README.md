# tonic-ping-pong
grpc in rust bidirectional streaming example using [tonic](https://github.com/hyperium/tonic/tree/master)

this example uses tokio's mpsc channels to stream messages between the client and server  
constructing a stream from iters that yield also works - but wasn't as neat/intuitive

an initial ping with value `0` is sent from client-> server, the server increments it and sends it back via response stream, which gets sent back to the server via the client req stream and this loops endlessly

## instructions to run
`cargo run --bin pingpong-server`
`cargo run --bin pingpong-client`

