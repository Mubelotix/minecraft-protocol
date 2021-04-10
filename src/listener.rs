use std::io::{Read, Write};
use crate::packets::{VarInt, serializer::MinecraftPacketPart, ConnectionState};

#[derive(Debug)]
pub enum ListenError {
    Custom(&'static str),
    Io(std::io::Error),
}

impl From<std::io::Error> for ListenError {
    fn from(e: std::io::Error) -> Self {
        ListenError::Io(e)
    }
}

impl From<&'static str> for ListenError {
    fn from(e: &'static str) -> Self {
        ListenError::Custom(e)
    }
}

pub fn read_packet(mut reader: impl Read, compression: Option<u32>, encryption: Option<&[u8]>) -> Result<Vec<u8>, ListenError> {
    let mut lenght: Vec<u8> = Vec::with_capacity(2);

    loop {
        if lenght.len() >= 5 {
            return Err("Lenght too long".into());
        }
        let mut byte = [0];
        reader.read_exact(&mut byte)?;
        lenght.push(byte[0]);
        if byte[0] < 0b1000_0000 {
            break;
        }
    }

    let lenght = VarInt::deserialize_uncompressed_minecraft_packet(lenght.as_mut_slice())?;
    if lenght.0 < 0 {
        return Err("Negative packet lenght!".into());
    }
    let len = lenght.0 as usize;
    let mut data: Vec<u8> = Vec::with_capacity(len);
    unsafe {data.set_len(len)}
    reader.read_exact(&mut data)?;

    if data.starts_with(&[25]) {
        if let Ok(message) = std::str::from_utf8(data.get(3..).unwrap()) {
            println!("{}", message);
        }
    }
    
    Ok(data)
}

#[test]
fn test() {
    use std::net::TcpStream;

    let mut stream = TcpStream::connect("127.0.0.1:25565").unwrap();

    stream.write_all(&crate::packets::handshake::ServerboundPacket::Hello {
        protocol_version: 754.into(),
        server_address: "127.0.0.1",
        server_port: 25565,
        next_state: crate::packets::ConnectionState::Login,
    }.serialize_uncompressed_minecraft_packet().unwrap()).unwrap();

    stream.write_all(&crate::packets::login::ServerboundPacket::LoginStart {
        username: "mubelotix",
    }.serialize_uncompressed_minecraft_packet().unwrap()).unwrap();

    let mut response = read_packet(&stream, None, None).unwrap();
    let response_packet = crate::packets::login::ClientboundPacket::deserialize_uncompressed_minecraft_packet(&mut response).unwrap();
    println!("{:?}", response_packet);

    use crate::packets::play_clientbound::ClientBoundPacket;

    loop {
        let mut packet_bytes = read_packet(&stream, None, None).unwrap();
        let packet = ClientBoundPacket::deserialize_uncompressed_minecraft_packet(&mut packet_bytes);
        if let Err(e) = packet {
            panic!("{} for {:?}", e, packet_bytes);
        }
    }
}
