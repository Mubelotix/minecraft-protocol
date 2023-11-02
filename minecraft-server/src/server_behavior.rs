use tokio::net::TcpListener;

use crate::prelude::*;

/// Message sent from the server to all player handlers
#[derive(Clone, Debug)]
pub enum ServerMessage {
    /// Message indicating a new tick has started
    Tick,
}

pub struct ServerBehavior {
    player_handlers: Vec<Task>,
    listener: TcpListener,
    message_receiver: BroadcastReceiver<ServerMessage>,
}

impl ServerBehavior {
    pub async fn init() -> ServerBehavior {
        let listener = TcpListener::bind("127.0.0.1:25567").await.expect("Failed to listen");
        let (sender, receiver) = broadcast_channel(100);

        // Send ticks to player handlers
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(Duration::from_millis(50));
            loop {
                tick.tick().await;
                let _ = sender.send(ServerMessage::Tick);
            }
        });

        ServerBehavior {
            listener,
            player_handlers: Vec::new(),
            message_receiver: receiver,
        }
    }

    pub fn poll(
        &mut self,
        cx: &mut Context<'_>
    ) -> Poll<()> {
        match self.listener.poll_accept(cx) {
            Ready(Ok((stream, addr))) => {
                if self.player_handlers.len() >= MAX_PLAYERS {
                    warn!("Server is full and failed to accept new player");
                } else {
                    debug!("Accepted connection from: {addr}");
                    let server_msg_rcvr = self.message_receiver.resubscribe();
                    self.player_handlers.push(Box::pin(handle_connection(stream, addr, server_msg_rcvr)));
                }
            }
            Ready(Err(e)) => error!("Failed to accept connection: {e}"),
            Pending => (),
        }

        for i in (0..self.player_handlers.len()).rev() {
            let handler = &mut self.player_handlers[i];
            match handler.as_mut().poll(cx) {
                Ready(_) => {
                    debug!("Player handler finished");
                    self.player_handlers.swap_remove(i);
                }
                Pending => (),
            }
        }

        Pending
    }
}
