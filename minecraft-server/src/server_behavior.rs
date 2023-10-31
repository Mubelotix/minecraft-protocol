use tokio::net::TcpListener;

use crate::prelude::*;

pub struct ServerBehavior {
    player_handlers: Vec<Task>,
    listener: TcpListener
}

impl ServerBehavior {
    pub async fn init() -> ServerBehavior {
        let listener = TcpListener::bind("127.0.0.1:25567").await.expect("Failed to listen");

        ServerBehavior {
            listener,
            player_handlers: Vec::new()
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
                    self.player_handlers.push(Box::pin(handle_player(stream, addr)));
                }
            }
            Ready(Err(e)) => error!("Failed to accept connection: {e}"),
            Pending => (),
        }

        for i in (0..self.player_handlers.len()).rev() {
            let handler = &mut self.player_handlers[i];
            match handler.as_mut().poll(cx) {
                Ready(()) => {
                    debug!("Player handler finished");
                    self.player_handlers.swap_remove(i);
                }
                Pending => (),
            }
        }

        Pending
    }
}
