use pb::ping_ponger_server::{PingPonger, PingPongerServer};
use pb::{Ping, Pong};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("pingpong.streaming");
}

#[derive(Debug)]
pub struct PingPongService {
    index: Arc<RwLock<u32>>,
}

#[tonic::async_trait]
impl PingPonger for PingPongService {
    type PingPongStream = ReceiverStream<Result<Pong, Status>>;
    async fn ping_pong(
        &self,
        request: Request<tonic::Streaming<Ping>>,
    ) -> Result<Response<Self::PingPongStream>, Status> {
        let mut req_stream = request.into_inner();
        let index = self.index.clone();
        let (tx, rx) = mpsc::channel(1000);

        tokio::spawn(async move {
            while let Some(ping) = req_stream.next().await {
                let ping = ping.unwrap();
                println!("Message recieved: {}", ping.message);
                let num_str = ping.message.split(':').next_back().unwrap();
                let num: u32 = num_str.trim().parse().unwrap();
                *index.write().unwrap() = num + 1;
                let pong = *index.read().unwrap();
                tx.send(Ok(Pong { pong })).await.unwrap();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10001".parse().unwrap();
    println!("PingPongServer listening on: {}", addr);
    let ping_ponger = PingPongService {
        index: Arc::new(RwLock::from(0)),
    };
    let service = PingPongerServer::new(ping_ponger);
    Server::builder()
        .add_service(service)
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}
