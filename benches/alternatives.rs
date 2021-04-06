#![feature(test)]

extern crate test;
use test::Bencher;

const LEVEL: &[u8] = include_bytes!("../src/nbt/test_data/level.dat");
const SERVERS: &[u8] = include_bytes!("../src/nbt/test_data/servers.dat");

#[bench]
fn minecraft_format(b: &mut Bencher) {
    /*let mut data = LEVEL.to_vec();
    let data = unsafe {
        let len = data.len();
        let data: &[u8] = &data;
        let data = data.as_ptr() as *mut u8;
        let data: &mut [u8] = std::slice::from_raw_parts_mut(data, len);
        data
    };*/
    
    b.iter(|| {
        let mut data = LEVEL.to_vec();
        minecraft_format::nbt::parse_nbt(&mut data);
    });
}

#[bench]
fn minecraft_format_servers(b: &mut Bencher) {
    b.iter(|| {
        let mut data = SERVERS.to_vec();
        minecraft_format::nbt::parse_nbt(&mut data);
    });
}
