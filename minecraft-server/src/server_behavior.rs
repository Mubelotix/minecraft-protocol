use tokio::net::TcpListener;

use crate::prelude::*;

/// Message sent from the server to all player handlers
#[derive(Clone, Debug)]
pub enum ServerMessage {
    /// Message indicating a new tick has started
    Tick,
}

pub struct ServerBehavior {
    world: &'static World,
    message_receiver: BroadcastReceiver<ServerMessage>,
}

impl ServerBehavior {
    pub async fn init() -> ServerBehavior {
        let listener = TcpListener::bind("127.0.0.1:25567").await.expect("Failed to listen");
        let (sender, receiver) = broadcast_channel(100);
        let world = Box::leak(Box::new(World::new(receiver.resubscribe())));

        // Send ticks to player handlers
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(Duration::from_millis(50));
            loop {
                tick.tick().await;
                let _ = sender.send(ServerMessage::Tick);
            }
        });

        // Accept incoming connections
        let world2: &World = world;
        let receiver2 = receiver.resubscribe();
        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                // TODO(security): Limit player count
                let server_msg_rcvr = receiver2.resubscribe();
                tokio::spawn(async move {
                    handle_connection(stream, addr, server_msg_rcvr, world2).await;
                });
            }
            error!("Listener couldn't listen anymore");
        });

        ServerBehavior {
            world,
            message_receiver: receiver,
        }
    }

    pub fn poll(
        &mut self,
        cx: &mut Context<'_>
    ) -> Poll<()> {
        Pending
    }
}
