pub use crate::{player_handler::*, server_behavior::*, position::*, map::*};
pub use log::{debug, error, info, trace, warn};
pub use minecraft_protocol::packets::{
    handshake::ServerboundPacket as HandshakeServerbound,
    login::{ClientboundPacket as LoginClientbound, ServerboundPacket as LoginServerbound},
    config::{ClientboundPacket as ConfigClientbound, ServerboundPacket as ConfigServerbound},
    status::{ClientboundPacket as StatusClientbound, ServerboundPacket as StatusServerbound},
    play_clientbound::ClientboundPacket as PlayClientbound,
    play_serverbound::ServerboundPacket as PlayServerbound, serializer::*, *,
};
pub use std::{
    pin::Pin,
    task::{
        Context,
        Poll::{self, *},
        Waker,
    },
};

pub const MAX_PLAYERS: usize = 1001;
