use super::*;

pub async fn receive_packet(stream: &mut TcpStream) -> Vec<u8> {
    let mut length: Vec<u8> = Vec::with_capacity(2);

    loop {
        if length.len() >= 5 {
            //return Err("length too long".into());
        }
        let mut byte = [0];
        stream.read_exact(&mut byte).await.unwrap();
        length.push(byte[0]);
        if byte[0] < 0b1000_0000 {
            break;
        }
    }

    let length = VarInt::deserialize_uncompressed_minecraft_packet(length.as_mut_slice()).unwrap();

    let mut data = Vec::with_capacity(length.0 as usize);
    unsafe { data.set_len(length.0 as usize); }
    stream.read_exact(&mut data).await.unwrap();

    data
}

pub async fn receive_packet_split(stream: &mut OwnedReadHalf) -> Vec<u8> {
    let mut length: Vec<u8> = Vec::with_capacity(2);

    loop {
        if length.len() >= 5 {
            //return Err("length too long".into());
        }
        let mut byte = [0];
        stream.read_exact(&mut byte).await.unwrap();
        length.push(byte[0]);
        if byte[0] < 0b1000_0000 {
            break;
        }
    }

    let length = VarInt::deserialize_uncompressed_minecraft_packet(length.as_mut_slice()).unwrap();

    let mut data = Vec::with_capacity(length.0 as usize);
    unsafe { data.set_len(length.0 as usize); }
    stream.read_exact(&mut data).await.unwrap();

    data
}

pub async fn send_packet_raw(stream: &mut TcpStream, packet: &[u8]) {
    let length = VarInt::from(packet.len());
    stream.write_all(length.serialize_minecraft_packet().unwrap().as_slice()).await.unwrap();
    stream.write_all(packet).await.unwrap();
    stream.flush().await.unwrap();
}

pub async fn send_packet_raw_split(stream: &mut OwnedWriteHalf, packet: &[u8]) {
    let length = VarInt::from(packet.len());
    stream.write_all(length.serialize_minecraft_packet().unwrap().as_slice()).await.unwrap();
    stream.write_all(packet).await.unwrap();
    stream.flush().await.unwrap();
}

pub async fn send_packet<'a, P: MinecraftPacketPart<'a>>(stream: &mut TcpStream, packet: P) {
    let packet = packet.serialize_minecraft_packet().unwrap();
    send_packet_raw(stream, packet.as_slice()).await;
}
