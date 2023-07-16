use std::f32::consts::PI;
use std::pin::Pin;

use pb::ping_ponger_client::PingPongerClient;
use pb::Ping;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::Channel;
use tonic::Request;

pub mod pb {
    tonic::include_proto!("pingpong.streaming");
}

async fn play_ping_pong(client: &mut PingPongerClient<Channel>) {
    let mut last_seen_pong: u32 = 0;
    let (tx, rx) = mpsc::channel(10000);
    let ack = ReceiverStream::new(rx);
    let response = client.ping_pong(Request::new(ack)).await.unwrap();

    let message = format!("last seen pong: {}", last_seen_pong);
    // kick start the pingpong with an init tx.send
    tx.send(Ping { message }).await.unwrap();
    let mut pong_stream = response.into_inner();
    while let Some(pong) = pong_stream.next().await {
        let pong = pong.unwrap();
        last_seen_pong = pong.pong;
        println!("last seen pong from server: {}", last_seen_pong);
        let message = format!("last seen pong: {}", last_seen_pong);
        tx.send(Ping { message }).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let mut client = PingPongerClient::connect("http://[::1]:10001")
        .await
        .unwrap();
    play_ping_pong(&mut client).await;
}
