#![allow(clippy::uninit_vec)]

mod player_handler;
mod server_behavior;
mod prelude;
mod map;
mod ecs;
mod world;

use crate::prelude::*;

struct Map {

}

fn min(a: f32, b: f32, c: f32) -> f32 {
    fn min2(a: f32, b: f32) -> f32 {
        if a < b {
            a
        } else {
            b
        }
    }
    min2(min2(a, b), c)
}

fn ray_cast(position: (f32, f32, f32), movement: (f32, f32, f32)) -> Vec<(isize, isize, isize)> {
    let final_position = ((position.0+movement.0) as isize, (position.1+movement.1) as isize, (position.2+movement.2) as isize);
    let mut result = Vec::new();
    let mut next_position = position;
    result.push((next_position.0 as isize, next_position.1 as isize, next_position.2 as isize));
    while result.last().unwrap() != &final_position {
        let next_x = if movement.0 > 0.0 { next_position.0.floor()+1.0 } else { next_position.0.floor()-1.0 };
        let next_y = if movement.1 > 0.0 { next_position.1.floor()+1.0 } else { next_position.1.floor()-1.0 };
        let next_z = if movement.2 > 0.0 { next_position.2.floor()+1.0 } else { next_position.2.floor()-1.0 };
        let x_dist = (next_x - next_position.0).abs();
        let y_dist = (next_y - next_position.1).abs();
        let z_dist = (next_z - next_position.2).abs();
        let x_time = x_dist / movement.0.abs();
        let y_time = y_dist / movement.1.abs();
        let z_time = z_dist / movement.2.abs();
        let time = min(x_time, y_time, z_time);
        println!("pos{next_position:?} dist({x_dist}, {y_dist}, {z_dist}) time({x_time}, {y_time}, {z_time}) time({time})");
        next_position = (next_position.0 + movement.0 * time, next_position.1 + movement.1 * time, next_position.2 + movement.2 * time);
        result.push((next_position.0 as isize, next_position.1 as isize, next_position.2 as isize));
    }
    result
}

struct ServerFuture {
    server: ServerBehavior,
}

impl std::future::Future for ServerFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.server.poll(cx)
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = ServerBehavior::init().await;
    let fut = ServerFuture { server };

    fut.await;
}
