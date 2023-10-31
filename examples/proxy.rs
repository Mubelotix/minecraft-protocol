use minecraft_protocol::{
    network::{read_packet, send_packet, NetworkError},
    packets::play_serverbound::ServerboundPacket as PlayServerbound,
    packets::play_clientbound::ClientboundPacket as PlayClientbound,
    packets::handshake::ServerboundPacket as HandshakeServerbound,
    packets::login::ServerboundPacket as LoginServerbound,
    packets::login::ClientboundPacket as LoginClientbound,
    packets::config::ClientboundPacket as ConfigClientbound,
    packets::config::ServerboundPacket as ConfigServerbound,
    packets::status::ServerboundPacket as StatusServerbound,
    packets::status::ClientboundPacket as StatusClientbound,
    *, components::chunk::Chunk,
};

use std::net::{TcpListener, TcpStream};

fn proxy_serverbound(client_stream: TcpStream, server_stream: TcpStream) -> Result<(), NetworkError> {
    loop {
        let packet = read_packet(&client_stream, None, None)?;
        if let Ok(packet) = PlayServerbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[35mclient\u{001b}[0m: {fpacket}");
            }
        } else if let Ok(packet) = LoginServerbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[35mclient\u{001b}[0m: {fpacket}");
            }
        } else if let Ok(packet) = HandshakeServerbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[35mclient\u{001b}[0m: {fpacket}");
            }
        } else if let Ok(packet) = ConfigServerbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[35mclient\u{001b}[0m: {fpacket}");
            }
        } else if let Ok(packet) = StatusServerbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[35mclient\u{001b}[0m: {fpacket}");
            }
        } else {
            let fpacket = if packet.len() > 200 {
                format!("{:?}...", &packet[..200])
            } else {
                format!("{:?}", packet)
            };
            let err = PlayServerbound::deserialize_uncompressed_minecraft_packet(&packet).unwrap_err();
            println!("\u{001b}[35mclient\u{001b}[0m: \u{001b}[31m{fpacket:?}\nwith {err}\u{001b}[0m");
        }

      

        send_packet(&server_stream, packet, None, None)?;
    }
}

fn proxy_clientbound(client_stream: TcpStream, server_stream: TcpStream) -> Result<(), NetworkError> {
    loop {
        let packet = read_packet(&server_stream, None, None)?;
        if let Ok(packet) = PlayClientbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[33mserver\u{001b}[0m: {fpacket}");
            }
            if let PlayClientbound::ChunkData { mut value } = packet {
                let chunks = match Chunk::from_data(&value.data.items) {
                    Ok(chunks) => chunks,
                    Err(e) => {
                        println!("Failed to deserialize chunks: {:?}", e);
                        continue;
                    }
                };
                let reserialized = match Chunk::into_data(chunks) {
                    Ok(reserialized) => reserialized,
                    Err(e) => {
                        println!("Failed to reserialize chunks: {:?}", e);
                        continue;
                    }
                };
                value.data.items = reserialized;
                let packet = match (PlayClientbound::ChunkData{value}.serialize_minecraft_packet()) {
                    Ok(packet) => packet,
                    Err(e) => {
                        println!("Failed to reserialize chunk packet: {:?}", e);
                        continue;
                    }
                };
                send_packet(&client_stream, packet, None, None)?;
                continue;
            }
        } else if let Ok(packet) = LoginClientbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[33mserver\u{001b}[0m: {fpacket}");
            }
        } else if let Ok(packet) = ConfigClientbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[33mserver\u{001b}[0m: {fpacket}");
            }
        } else if let Ok(packet) = StatusClientbound::deserialize_uncompressed_minecraft_packet(&packet) {
            #[cfg(feature = "all-packets")]
            {
                let mut fpacket = format!("{:?}", packet);
                if fpacket.len() > 400 {
                    fpacket.truncate(400);
                    fpacket.push_str("...");
                }
                println!("\u{001b}[33mserver\u{001b}[0m: {fpacket}");
            }
        } else {
            let fpacket = if packet.len() > 200 {
                format!("{:?}...", &packet[..200])
            } else {
                format!("{:?}", packet)
            };
            let err = PlayClientbound::deserialize_uncompressed_minecraft_packet(&packet).unwrap_err();
            println!("\u{001b}[33mserver\u{001b}[0m: \u{001b}[31m{fpacket:?}\nwith {err}\u{001b}[0m");
        }

        send_packet(&client_stream, packet, None, None)?;
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25566").unwrap();

    loop {
        match listener.accept() {
            Ok((client_stream, addr)) => {
                println!("new client: {addr:?}");
                let server_stream = TcpStream::connect("127.0.0.1:25565").unwrap();

                let client_stream2 = client_stream.try_clone().unwrap();
                let server_stream2 = server_stream.try_clone().unwrap();
                let handle1 = std::thread::spawn(move || proxy_serverbound(client_stream, server_stream));
                let handle2 = std::thread::spawn(move || proxy_clientbound(client_stream2, server_stream2));
                let _ = handle1.join().unwrap();
                let _ = handle2.join().unwrap();
                println!("client disconnected: {addr:?}")
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}
