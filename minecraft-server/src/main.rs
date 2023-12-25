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
    #[cfg(feature = "trace")]
    #[global_allocator]
    static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
        tracy_client::ProfiledAllocator::new(std::alloc::System, 100);

    use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};

    let subscriber = Registry::default()
                                .with(fmt::layer());
    #[cfg(feature = "trace")]
    let subscriber = subscriber
        .with(tracing_tracy::TracyLayer::new());
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting up tracing");
        
    env_logger::init();

    let server = ServerBehavior::init().await;
    let fut = ServerFuture { server };

    fut.await;
}
