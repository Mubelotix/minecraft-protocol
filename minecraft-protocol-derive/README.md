# minecraft-protocol-derive

Procedural macros to make your structs compatible with the Minecraft protocol. 

This crate aims to make the development of Minecraft protocol libraries easier.  
There is already [a complete Minecraft protocol implementation](https://github.com/Mubelotix/minecraft-protocol) using this crate, but you could also [make your own](https://wiki.vg/Main_Page).

## Usage

This crate requires you to declare a `MinecraftPacketPart` trait (see [tests](https://github.com/Mubelotix/minecraft-protocol-derive/tree/main/tests) for examples).  
The name of the derive macros is the same and can be used to implement the trait automatically.  
It can still be implemented manually for complex types.

```rust
#[derive(MinecraftPacketPart)]
struct Entity {
    entity_id: VarInt,
    x: f64,
    y: f64,
    z: f64,
}
```

You can also nest your different structures.

```rust
#[derive(MinecraftPacketPart)]
struct Entity {
    entity_id: VarInt,
    position: Position,
}

#[derive(MinecraftPacketPart)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}
```

Note that you need to implement the `MinecraftPacketPart` trait for all the primitive types.
The `VarInt` type is a common signed integer type.
See [the wiki](https://wiki.vg/Protocol#VarInt_and_VarLong) for help with the implementation.

## Enums

Enums are supported as long as you use named fields.

```rust
#[derive(MinecraftPacketPart)]
enum Packet {
    Login {
        username: String,
    },
    MoveTo {
        x: f64,
        y: f64,
        z: f64,
    }
    Ping,
}
```

For enums exclusively composed of unit variants, using `#[minecraft_enum]` is recommended since its implementation is more lightweight and optimized.

Since numeric IDs represent enums in the protocol, each variant has an associated value (its corresponding ID).
It can be specified explicitly or inferred.
A variant with no specified ID will take the ID of the previous variant, incremented by 1.
The first variant matches 0 if no ID is specified.

You can specify the type of the numeric ID in the macro parameters.
If missing, `VarInt` will be used.

```rust
#[minecraft_enum(u8)]
enum EntityType {
    Player,     // = 0 (inferred)
    Villager,
    Animal = 3, // Specify the corresponding ID like this.
    Monster,    // = 4 (inferred, 3+1)
    Minecart,
}
```

It is also possible to set the type and value of the numeric IDs with the derive syntax.

The same inference rules are applied.


```rust
#[derive(MinecraftPacketPart)]
#[discriminant(u8)] // define the type of the numeric ID of the variants
enum Packet {
    Login {
        username: String,
    },
    #[value = 5] // define the ID corresponding to the MoveTo variant
    MoveTo {
        x: f64,
        y: f64,
        z: f64,
    }
    Ping,
}
```

## Lifetimes

Lifetimes are supported everywhere.
The only limitation is that you cannot have more than one lifetime.

It is easy to implement zero-copy deserialization by using references.

```rust
#[derive(MinecraftPacketPart)]
struct Player<'a> {
    username: &'a str,
    data: &'a [u8],
}
```
