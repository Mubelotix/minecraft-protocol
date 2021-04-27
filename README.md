# minecraft-protocol

A library crate empowering everyone to build Minecraft clients and servers.

This crate is capable of handling the parsing and creation of Minecraft packets.
It contains a complete implementation of the latest Minecraft protocol, including [the NBT data format](https://wiki.vg/NBT).
This crate also embeds [exhaustive lists](https://github.com/PrismarineJS/minecraft-data) of blocks and entities, along with their properties.

The work relies on and contributes to a [community reverse engineering effort](https://wiki.vg).
I have tested the whole crate on real-world Minecraft servers and successfully made a miner bot.

## Usage

This crate is low-level since you will have to manage the network and learn about [how the protocol works](https://wiki.vg).
There are WIP helper functions for reading and writing packets over a `TcpStream`.
There is currently no support for encryption and compression.

You can serialize and deserialize any struct of this library like this:

```rust
// Serialize
let packet = ClientboundPacket::UpdateHealth {
    health: 20.0,
    food: VarInt(10),
    food_saturation: 2.0,
};
let serialized_packet: Vec<u8> = packet.serialize_minecraft_packet().unwrap();

// Deserialize
let raw_packet = [68, 160, 129, 2, 0, 0, 0, 2, 5, 0, 6, 18, 0, 4, 7, 0, 15, 13, 10, 14, 0, 0, 12, 1, 0, 13, 10, 0, 8, 2, 66, 32, 0, 0, 9, 1, 0, 11, 1, 0, 10, 7, 0, 1, 1, 172, 2, 3, 7, 0, 7, 0, 0, 5, 7, 0, 16, 7, 0, 17, 7, 0, 255];
let parsed_packet = ClientboundPacket::deserialize_minecraft_packet(&raw_packet).unwrap();
```

## Internal design

This crate uses procedural macros to generate most of the parsing of composed structs.
These macros are defined in [this crate](https://github.com/Mubelotix/minecraft-protocol-derive). Note that this is **the only dependency**.

As you can see, specifying new types is child's play:

```rust
#[derive(MinecraftPacketPart)]
struct Entity {
    entity_id: VarInt,
    x: f64,
    y: f64,
    z: f64,
}
```

This library consists of a bunch of structs (like this one) nested in each other.
See [Mubelotix/minecraft-protocol-derive](https://github.com/Mubelotix/minecraft-protocol-derive) for more information.

## State the Minecraft + Rust ecosystem

There are many library crates for Minecraft, but they are often incomplete and outdated.
This crate aims to be complete and easy to maintain. Thanks to the use of macros and build scripts, there is little parsing to implement by hand.

However, there is currently no high-level Minecraft client or server library.
It would be amazing to have a batteries-included Minecraft client framework.
It could handle packets, environment state, and provide easy-to-use functions.
That would allow client and bot creators to focus on the logic.
Minecraft isn't that complex, but there is a surprisingly large amount of content to understand in the protocol.
Despite the quality of [the wiki](https://wiki.vg), it may still be a pain point for beginners.
Thus, feel free to pursue the efforts on Minecraft in Rust as it will be highly appreciated.
