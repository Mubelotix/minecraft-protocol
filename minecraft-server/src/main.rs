#![allow(clippy::uninit_vec)]

mod player_handler;
mod server_behavior;
mod prelude;
mod world;
mod entities;

use crate::prelude::*;


struct ServerFuture {
    server: ServerBehavior,
}

impl std::future::Future for ServerFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.server.poll(cx)
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = ServerBehavior::init().await;
    let fut = ServerFuture { server };

    fut.await;
}
