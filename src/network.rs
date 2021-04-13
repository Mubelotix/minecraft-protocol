use crate::packets::{serializer::MinecraftPacketPart, VarInt};
use std::io::{Read, Write};

#[derive(Debug)]
pub enum NetworkError {
    Custom(&'static str),
    Io(std::io::Error),
}

impl From<std::io::Error> for NetworkError {
    fn from(e: std::io::Error) -> Self {
        NetworkError::Io(e)
    }
}

impl From<&'static str> for NetworkError {
    fn from(e: &'static str) -> Self {
        NetworkError::Custom(e)
    }
}

pub fn read_packet(
    mut reader: impl Read,
    _compression: Option<u32>,
    _encryption: Option<&[u8]>,
) -> Result<Vec<u8>, NetworkError> {
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
    unsafe { data.set_len(len) }
    reader.read_exact(&mut data)?;

    if data.starts_with(&[25]) {
        if let Ok(message) = std::str::from_utf8(data.get(3..).unwrap()) {
            println!("{}", message);
        }
    }

    Ok(data)
}

pub fn send_packet<'a>(
    mut writer: impl Write,
    packet: Vec<u8>,
    compression: Option<u32>,
    encryption: Option<&[u8]>,
) -> Result<(), NetworkError> {
    let mut packet_prefix = Vec::new();
    match compression {
        None => {
            let len = VarInt(packet.len() as i32);
            len.serialize_minecraft_packet_part(&mut packet_prefix)?;
        }
        Some(_threshold) => {
            unimplemented!("compression")
        }
    }
    match encryption {
        None => (),
        Some(_key) => {
            unimplemented!("encryption")
        }
    }
    writer.write_all(&packet_prefix)?;
    writer.write_all(&packet)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use std::net::TcpStream;

        let mut stream = TcpStream::connect("127.0.0.1:25565").unwrap();

        send_packet(
            &mut stream,
            crate::packets::handshake::ServerboundPacket::Hello {
                protocol_version: 754.into(),
                server_address: "127.0.0.1",
                server_port: 25565,
                next_state: crate::packets::ConnectionState::Login,
            }.serialize_minecraft_packet().unwrap(),
            None,
            None,
        )
        .unwrap();

        send_packet(
            &mut stream,
            crate::packets::login::ServerboundPacket::LoginStart { username: "bot2" }.serialize_minecraft_packet().unwrap(),
            None,
            None,
        )
        .unwrap();

        let mut response = read_packet(&stream, None, None).unwrap();
        let response_packet =
            crate::packets::login::ClientboundPacket::deserialize_uncompressed_minecraft_packet(
                &mut response,
            )
            .unwrap();
        println!("{:?}", response_packet);

        use crate::packets::{
            play_clientbound::ClientboundPacket, play_serverbound::ServerboundPacket,
        };

        loop {
            let mut packet_bytes = read_packet(&stream, None, None).unwrap();
            let packet =
                ClientboundPacket::deserialize_uncompressed_minecraft_packet(&mut packet_bytes);
            let packet = match packet {
                Ok(packet) => packet,
                Err(e) => panic!("{} for {:?}", e, packet_bytes),
            };
            match packet {
                ClientboundPacket::KeepAlive { keep_alive_id } => {
                    send_packet(
                        &mut stream,
                        ServerboundPacket::KeepAlive { keep_alive_id }.serialize_minecraft_packet().unwrap(),
                        None,
                        None,
                    )
                    .unwrap();
                    println!("pong!");
                }
                ClientboundPacket::Advancements { .. } => {
                    println!("Advancements parsed successfully!")
                }
                ClientboundPacket::ChunkData { mut value } => {
                    value.deserialize_chunk_sections().unwrap();
                    println!("chunk parsed successfully!")
                }
                ClientboundPacket::ChatMessage {
                    message,
                    position: _,
                    sender,
                } => {
                    println!("{}: {}", sender, message);
                }
                _ => (),
            }
        }
    }
}
