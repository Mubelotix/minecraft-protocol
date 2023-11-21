use tokio::net::TcpListener;

use crate::prelude::*;

pub struct ServerBehavior {
    world: &'static World,
}

impl ServerBehavior {
    pub async fn init() -> ServerBehavior {
        let listener = TcpListener::bind("127.0.0.1:25567").await.expect("Failed to listen");
        let world = World::new();

        // Send ticks to player handlers
        let world2: &World = world;
        tokio::spawn(async move {
            let mut tick_id = 0;
            let mut tick = tokio::time::interval(Duration::from_millis(50));
            loop {
                tick.tick().await;
                world2.tick(tick_id).await;
                tick_id += 1;
            }
        });

        // Accept incoming connections
        let world2: &World = world;
        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                // TODO(security): Limit player count
                tokio::spawn(async move {
                    handle_connection(stream, addr, world2).await;
                });
            }
            error!("Listener couldn't listen anymore");
        });

        ServerBehavior {
            world,
        }
    }

    pub fn poll(
        &mut self,
        cx: &mut Context<'_>
    ) -> Poll<()> {
        Pending
    }
}
