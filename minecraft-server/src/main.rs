#![allow(clippy::uninit_vec)]

mod player_handler;
mod server_behavior;
mod prelude;
mod world;
mod entities;

use crate::prelude::*;


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

fn min_options(a: Option<f32>, b: Option<f32>, c: Option<f32>) -> Option<f32> {
    match (a, b, c) {
        (Some(a), Some(b), Some(c)) => Some(min(a, b, c)),
        (Some(a), Some(b), None) => Some(min(a, b, 1.0)),
        (Some(a), None, Some(c)) => Some(min(a, 1.0, c)),
        (None, Some(b), Some(c)) => Some(min(1.0, b, c)),
        (Some(a), None, None) => Some(a),
        (None, Some(b), None) => Some(b),
        (None, None, Some(c)) => Some(c),
        (None, None, None) => None,
    }
}

fn min_options2(a: Option<f32>, b: Option<f32>) -> Option<f32> {
    match (a, b) {
        (Some(a), Some(b)) => Some(min(a, b, 1.0)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

struct CollisionShape {
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
}

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

struct PointIter<'a> {
    shape: &'a CollisionShape,
    index: usize,
}

impl<'a> Iterator for PointIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 8 {
            let result = Point {
                x: if self.index & 1 == 0 { self.shape.x1 } else { self.shape.x2 },
                y: if self.index & 2 == 0 { self.shape.y1 } else { self.shape.y2 },
                z: if self.index & 4 == 0 { self.shape.z1 } else { self.shape.z2 },
            };
            Some(result)
        } else {
            None
        }
    }
}

impl CollisionShape {
    const fn points(&self) -> PointIter {
        PointIter {
            shape: self,
            index: 0,
        }
    }
}

struct Translation {
    x: f32,
    y: f32,
    z: f32,
}

fn is_inside(shape: &CollisionShape, point: Point) -> bool {
    (shape.x1..=shape.x2).contains(&point.x) && (shape.y1..=shape.y2).contains(&point.y) && (shape.z1..=shape.z2).contains(&point.z)
}

fn translation_limit_y(shape: &CollisionShape, translation: &Translation, point: &Point) -> Option<f32> {
    let y = if translation.y < 0.0 { shape.y1 } else { shape.y2 };
    let translated_ratio = (y - shape.y1) / translation.y;
    if !(0.0..=1.0).contains(&translated_ratio) {
        return None;
    }
    let translated_x1 = shape.x1 + (shape.x2 - shape.x1) * translated_ratio;
    let translated_x2 = shape.x2 + (shape.x2 - shape.x1) * translated_ratio;
    let translated_z1 = shape.z1 + (shape.z2 - shape.z1) * translated_ratio;
    let translated_z2 = shape.z2 + (shape.z2 - shape.z1) * translated_ratio;
    if (translated_x1..=translated_x2).contains(&point.x) && (translated_z1..=translated_z2).contains(&point.z) {
        Some(translated_ratio)
    } else {
        None
    }
}

fn translation_limit_x(shape: &CollisionShape, translation: &Translation, point: &Point) -> Option<f32> {
    let x = if translation.x < 0.0 { shape.x1 } else { shape.x2 };
    let translated_ratio = (x - shape.x1) / translation.x;
    if !(0.0..=1.0).contains(&translated_ratio) {
        return None;
    }
    let translated_y1 = shape.y1 + (shape.y2 - shape.y1) * translated_ratio;
    let translated_y2 = shape.y2 + (shape.y2 - shape.y1) * translated_ratio;
    let translated_z1 = shape.z1 + (shape.z2 - shape.z1) * translated_ratio;
    let translated_z2 = shape.z2 + (shape.z2 - shape.z1) * translated_ratio;
    if (translated_y1..=translated_y2).contains(&point.y) && (translated_z1..=translated_z2).contains(&point.z) {
        Some(translated_ratio)
    } else {
        None
    }
}

fn translation_limit_z(shape: &CollisionShape, translation: &Translation, point: &Point) -> Option<f32> {
    let z = if translation.z < 0.0 { shape.z1 } else { shape.z2 };
    let translated_ratio = (z - shape.z1) / translation.z;
    if !(0.0..=1.0).contains(&translated_ratio) {
        return None;
    }
    let translated_x1 = shape.x1 + (shape.x2 - shape.x1) * translated_ratio;
    let translated_x2 = shape.x2 + (shape.x2 - shape.x1) * translated_ratio;
    let translated_y1 = shape.y1 + (shape.y2 - shape.y1) * translated_ratio;
    let translated_y2 = shape.y2 + (shape.y2 - shape.y1) * translated_ratio;
    if (translated_x1..=translated_x2).contains(&point.x) && (translated_y1..=translated_y2).contains(&point.y) {
        Some(translated_ratio)
    } else {
        None
    }
}

fn translation_limit(shape: &CollisionShape, translation: &Translation, point: &Point) -> Option<f32> {
    min_options(
        translation_limit_x(shape, translation, point),
        translation_limit_y(shape, translation, point),
        translation_limit_z(shape, translation, point)
    )
}

fn collide(translating: CollisionShape, translation: Translation, obstacle: CollisionShape) -> Option<Translation> {
    let mut limit = None;

    for point in obstacle.points() {
        limit = min_options2(limit, translation_limit(&translating, &translation, &point));
    }

    limit.map(|limit| Translation {
        x: translation.x * limit,
        y: translation.y * limit,
        z: translation.z * limit,
    })
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
