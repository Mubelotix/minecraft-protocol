use minecraft_protocol::{
    network::{read_packet, send_packet, NetworkError},
    packets::play_serverbound::ServerboundPacket as PlayServerbound,
    packets::play_clientbound::ClientboundPacket as PlayClientbound,
    packets::handshake::ServerboundPacket as HandshakeServerbound,
    packets::login::ServerboundPacket as LoginServerbound,
    packets::login::ClientboundPacket as LoginClientbound,
    *,
};

use std::net::{TcpListener, TcpStream};

fn proxy_serverbound(client_stream: TcpStream, server_stream: TcpStream) -> Result<(), NetworkError> {
    loop {
        let packet = read_packet(&client_stream, None, None)?;
        if let Ok(packet) = PlayServerbound::deserialize_uncompressed_minecraft_packet(&packet) {
            println!("\u{001b}[35mclient\u{001b}[0m: {packet:?}");
        } else if let Ok(packet) = LoginServerbound::deserialize_uncompressed_minecraft_packet(&packet) {
            println!("\u{001b}[35mclient\u{001b}[0m: {packet:?}");
        } else if let Ok(packet) = HandshakeServerbound::deserialize_uncompressed_minecraft_packet(&packet) {
            println!("\u{001b}[35mclient\u{001b}[0m: {packet:?}");
        } else {
            println!("\u{001b}[35mclient\u{001b}[0m: \u{001b}[31m{packet:?}\u{001b}[0m");
        }

        send_packet(&server_stream, packet, None, None)?;
    }
}

fn proxy_clientbound(client_stream: TcpStream, server_stream: TcpStream) -> Result<(), NetworkError> {
    loop {
        let packet = read_packet(&server_stream, None, None)?;
        if let Ok(packet) = PlayClientbound::deserialize_uncompressed_minecraft_packet(&packet) {
            println!("\u{001b}[33mserver\u{001b}[0m: {packet:?}");
        } else if let Ok(packet) = LoginClientbound::deserialize_uncompressed_minecraft_packet(&packet) {
            println!("\u{001b}[33mserver\u{001b}[0m: {packet:?}\u{001b}[0m");
        } else {
            println!("\u{001b}[33mserver\u{001b}[0m: \u{001b}[31m{packet:?}\u{001b}[0m");
        }

        send_packet(&client_stream, packet, None, None)?;
    }
}

#[test]
fn proxy() {
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