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

#[derive(Debug, Clone, PartialEq)]
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
            self.index += 1;
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

#[derive(Debug, Clone, PartialEq)]
struct Translation {
    x: f32,
    y: f32,
    z: f32,
}

impl std::ops::Add<Translation> for Translation {
    type Output = Translation;

    fn add(self, rhs: Translation) -> Self::Output {
        Translation {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Translation> for CollisionShape {
    type Output = CollisionShape;

    fn add(self, rhs: Translation) -> Self::Output {
        CollisionShape {
            x1: self.x1 + rhs.x,
            y1: self.y1 + rhs.y,
            z1: self.z1 + rhs.z,
            x2: self.x2 + rhs.x,
            y2: self.y2 + rhs.y,
            z2: self.z2 + rhs.z,
        }
    }
}

fn is_inside(shape: &CollisionShape, point: Point) -> bool {
    (shape.x1..=shape.x2).contains(&point.x) && (shape.y1..=shape.y2).contains(&point.y) && (shape.z1..=shape.z2).contains(&point.z)
}

fn translation_limit_y(shape: &CollisionShape, translation: &Translation, point: &Point) -> Option<f32> {
    if translation.y == 0.0 {
        return None;
    }
    let y = if translation.y < 0.0 { shape.y1 } else { shape.y2 };
    let translated_ratio = (point.y - y) / translation.y;
    if translated_ratio >= 1.0 {
        return None;
    } else if translated_ratio <= 0.0 {
        return Some(0.0)
    }
    let translated_x1 = shape.x1 + translation.x * translated_ratio;
    let translated_x2 = shape.x2 + translation.x * translated_ratio;
    let translated_z1 = shape.z1 + translation.z * translated_ratio;
    let translated_z2 = shape.z2 + translation.z * translated_ratio;
    if (translated_x1..=translated_x2).contains(&point.x) && (translated_z1..=translated_z2).contains(&point.z) {
        Some(translated_ratio)
    } else {
        None
    }
}

fn translation_limit_x(shape: &CollisionShape, translation: &Translation, point: &Point) -> Option<f32> {
    if translation.x == 0.0 {
        return None;
    }
    let x = if translation.x < 0.0 { shape.x1 } else { shape.x2 };
    let translated_ratio = (point.x - x) / translation.x;
    if translated_ratio >= 1.0 {
        return None;
    } else if translated_ratio <= 0.0 {
        return Some(0.0)
    }
    let translated_y1 = shape.y1 + translation.y * translated_ratio;
    let translated_y2 = shape.y2 + translation.y * translated_ratio;
    let translated_z1 = shape.z1 + translation.z * translated_ratio;
    let translated_z2 = shape.z2 + translation.z * translated_ratio;
    if (translated_y1..=translated_y2).contains(&point.y) && (translated_z1..=translated_z2).contains(&point.z) {
        Some(translated_ratio)
    } else {
        None
    }
}

fn translation_limit_z(shape: &CollisionShape, translation: &Translation, point: &Point) -> Option<f32> {
    if translation.z == 0.0 {
        return None;
    }
    let z = if translation.z < 0.0 { shape.z1 } else { shape.z2 };
    let translated_ratio = (point.z - z) / translation.z;
    if translated_ratio >= 1.0 {
        return None;
    } else if translated_ratio <= 0.0 {
        return Some(0.0)
    }
    let translated_x1 = shape.x1 + translation.x * translated_ratio;
    let translated_x2 = shape.x2 + translation.x * translated_ratio;
    let translated_y1 = shape.y1 + translation.y * translated_ratio;
    let translated_y2 = shape.y2 + translation.y * translated_ratio;
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

fn collide(translating: &CollisionShape, translation: &Translation, obstacle: &CollisionShape) -> Option<Translation> {
    let mut limit = None;

    for point in obstacle.points() {
        limit = min_options2(limit, translation_limit(translating, translation, &point));
        if limit.map(|l| l==0.0).unwrap_or(false) {
            break;
        }
    }

    limit.map(|limit| Translation {
        x: translation.x * limit,
        y: translation.y * limit,
        z: translation.z * limit,
    })
}

#[test]
fn test() {
    let shape1 = CollisionShape {
        x1: 0.0,
        y1: 0.0,
        z1: 0.0,
        x2: 1.0,
        y2: 1.0,
        z2: 1.0,
    };

    // Boxes are just next to each other and pushing against each other
    let shape2 = shape1.clone() + Translation { x: 1.0, y: 0.0, z: 0.0 };
    let translation = Translation { x: -1.0, y: 0.0, z: 0.0 };
    assert_eq!(collide(&shape2, &translation, &shape1), Some(Translation { x: 0.0, y: 0.0, z: 0.0 }));

    // Boxes are one block away but one comes and pushes the other
    let shape2 = shape1.clone() + Translation { x: 2.0, y: 0.0, z: 0.0 };
    let translation = Translation { x: -2.0, y: 0.0, z: 0.0 };
    assert_eq!(collide(&shape2, &translation, &shape1), Some(Translation { x: -1.0, y: 0.0, z: 0.0 }));

    // The other way around
    let shape2 = shape1.clone() + Translation { x: -2.0, y: 0.0, z: 0.0 };
    let translation = Translation { x: 2.0, y: 0.0, z: 0.0 };
    assert_eq!(collide(&shape2, &translation, &shape1), Some(Translation { x: 1.0, y: 0.0, z: 0.0 }));

    // From top
    let shape2 = shape1.clone() + Translation { x: 0.0, y: 2.0, z: 0.0 };
    let translation = Translation { x: 0.0, y: -2.0, z: 0.0 };
    assert_eq!(collide(&shape2, &translation, &shape1), Some(Translation { x: 0.0, y: -1.0, z: 0.0 }));

    // On last axis
    let shape2 = shape1.clone() + Translation { x: 0.0, y: 0.0, z: 2.0 };
    let translation = Translation { x: 0.0, y: 0.0, z: -2.0 };
    assert_eq!(collide(&shape2, &translation, &shape1), Some(Translation { x: 0.0, y: 0.0, z: -1.0 }));

    // Colliding on corner
    let shape2 = shape1.clone() + Translation { x: 2.0, y: 2.0, z: 2.0 };
    let translation = Translation { x: -2.0, y: -2.0, z: -2.0 };
    assert_eq!(collide(&shape2, &translation, &shape1), Some(Translation { x: -1.0, y: -1.0, z: -1.0 }));

    // Colliding with offset on other axis
    let shape2 = shape1.clone() + Translation { x: 2.0, y: 0.5, z: 0.0 };
    let translation = Translation { x: -2.0, y: 0.0, z: 0.0 };
    assert_eq!(collide(&shape2, &translation, &shape1), Some(Translation { x: -1.0, y: 0.0, z: 0.0 }));

    // Colliding when already inside
    let shape2 = shape1.clone() + Translation { x: 0.5, y: 0.5, z: 0.5 };
    let translation = Translation { x: -0.5, y: -0.5, z: -0.5 };
    assert_eq!(collide(&shape2, &translation, &shape1), Some(Translation { x: 0.0, y: 0.0, z: 0.0 }));
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
