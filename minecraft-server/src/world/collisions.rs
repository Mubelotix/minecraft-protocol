fn min2(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn min(a: f32, b: f32, c: f32) -> f32 {
    min2(min2(a, b), c)
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

    // TODO(perf): Return an iterator yielding blocks instead of a vec of blocks
    fn containing_blocks(&self) -> Vec<BlockPosition> {
        let mut result = Vec::new();
        for x in self.x1.floor() as i32..=self.x2.floor() as i32 {
            for y in self.y1.floor() as i32..=self.y2.floor() as i32 {
                for z in self.z1.floor() as i32..=self.z2.floor() as i32 {
                    let block = BlockPosition { x, y, z };
                    result.push(block);
                }
            }
        }
        result
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

impl std::ops::Add<&Translation> for CollisionShape {
    type Output = CollisionShape;

    fn add(self, rhs: &Translation) -> Self::Output {
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

impl std::ops::AddAssign<&Translation> for CollisionShape {
    fn add_assign(&mut self, rhs: &Translation) {
        self.x1 += rhs.x;
        self.y1 += rhs.y;
        self.z1 += rhs.z;
        self.x2 += rhs.x;
        self.y2 += rhs.y;
        self.z2 += rhs.z;
    }
}

impl std::ops::Mul<f32> for Translation {
    type Output = Translation;

    fn mul(self, rhs: f32) -> Self::Output {
        Translation {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

fn is_inside(shape: &CollisionShape, point: Point) -> bool {
    (shape.x1..=shape.x2).contains(&point.x) && (shape.y1..=shape.y2).contains(&point.y) && (shape.z1..=shape.z2).contains(&point.z)
}

fn translation_limit_y(shape: &CollisionShape, translation: &Translation, point: &Point) -> f32 {
    if translation.y == 0.0 {
        return 1.0;
    }
    let y = if translation.y < 0.0 { shape.y1 } else { shape.y2 };
    let translated_ratio = (point.y - y) / translation.y;
    if translated_ratio >= 1.0 {
        return 1.0;
    } else if translated_ratio <= 0.0 {
        return 0.0;
    }
    let translated_x1 = shape.x1 + translation.x * translated_ratio;
    let translated_x2 = shape.x2 + translation.x * translated_ratio;
    let translated_z1 = shape.z1 + translation.z * translated_ratio;
    let translated_z2 = shape.z2 + translation.z * translated_ratio;
    if (translated_x1..=translated_x2).contains(&point.x) && (translated_z1..=translated_z2).contains(&point.z) {
        translated_ratio
    } else {
        1.0
    }
}

fn translation_limit_x(shape: &CollisionShape, translation: &Translation, point: &Point) -> f32 {
    if translation.x == 0.0 {
        return 1.0;
    }
    let x = if translation.x < 0.0 { shape.x1 } else { shape.x2 };
    let translated_ratio = (point.x - x) / translation.x;
    if translated_ratio >= 1.0 {
        return 1.0;
    } else if translated_ratio <= 0.0 {
        return 0.0;
    }
    let translated_y1 = shape.y1 + translation.y * translated_ratio;
    let translated_y2 = shape.y2 + translation.y * translated_ratio;
    let translated_z1 = shape.z1 + translation.z * translated_ratio;
    let translated_z2 = shape.z2 + translation.z * translated_ratio;
    if (translated_y1..=translated_y2).contains(&point.y) && (translated_z1..=translated_z2).contains(&point.z) {
        translated_ratio
    } else {
        1.0
    }
}

fn translation_limit_z(shape: &CollisionShape, translation: &Translation, point: &Point) -> f32 {
    if translation.z == 0.0 {
        return 1.0;
    }
    let z = if translation.z < 0.0 { shape.z1 } else { shape.z2 };
    let translated_ratio = (point.z - z) / translation.z;
    if translated_ratio >= 1.0 {
        return 1.0;
    } else if translated_ratio <= 0.0 {
        return 0.0;
    }
    let translated_x1 = shape.x1 + translation.x * translated_ratio;
    let translated_x2 = shape.x2 + translation.x * translated_ratio;
    let translated_y1 = shape.y1 + translation.y * translated_ratio;
    let translated_y2 = shape.y2 + translation.y * translated_ratio;
    if (translated_x1..=translated_x2).contains(&point.x) && (translated_y1..=translated_y2).contains(&point.y) {
        translated_ratio
    } else {
        1.0
    }
}

fn translation_limit(shape: &CollisionShape, translation: &Translation, point: &Point) -> f32 {
    min(
        translation_limit_x(shape, translation, point),
        translation_limit_y(shape, translation, point),
        translation_limit_z(shape, translation, point)
    )
}

fn restrict(translating: &CollisionShape, translation: &mut Translation, obstacle: &CollisionShape) {
    let mut limit = 1.0;

    for point in obstacle.points() {
        limit = min2(limit, translation_limit(translating, translation, &point));
        if limit == 0.0 {
            break;
        }
    }

    translation.x *= limit;
    translation.y *= limit;
    translation.z *= limit;
}

fn ray_cast(position: CollisionShape, movement: Translation) -> Vec<Translation> {
    let final_position = position.clone() + &movement;
    let mut result = Vec::new();
    let mut next_position = position.clone();
    //result.extend(position.containing_blocks().into_iter());
    while next_position != final_position {
        let x_dist = if movement.x > 0.0 {
            let next_x = next_position.x1.floor()+1.0;
            (next_x - next_position.x1).abs()
        } else {
            let next_x = next_position.x2.floor()-1.0;
            (next_x - next_position.x2).abs()
        };
        let y_dist = if movement.y > 0.0 {
            let next_y = next_position.y1.floor()+1.0;
            (next_y - next_position.y1).abs()
        } else {
            let next_y = next_position.y2.floor()-1.0;
            (next_y - next_position.y2).abs()
        };
        let z_dist = if movement.z > 0.0 {
            let next_z = next_position.z1.floor()+1.0;
            (next_z - next_position.z1).abs()
        } else {
            let next_z = next_position.z2.floor()-1.0;
            (next_z - next_position.z2).abs()
        };
        let x_time = x_dist / movement.x.abs();
        let y_time = y_dist / movement.y.abs();
        let z_time = z_dist / movement.z.abs();
        let time = min(x_time, y_time, z_time);
        println!("pos{next_position:?} dist({x_dist}, {y_dist}, {z_dist}) time({x_time}, {y_time}, {z_time}) time({time})");
        let mini_translation = movement.clone() * time;
        next_position += &mini_translation;
        result.push(mini_translation);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut translation = Translation { x: -1.0, y: 0.0, z: 0.0 };
        restrict(&shape2, &mut translation, &shape1);
        assert_eq!(translation, Translation { x: 0.0, y: 0.0, z: 0.0 });

        // Boxes are one block away but one comes and pushes the other
        let shape2 = shape1.clone() + Translation { x: 2.0, y: 0.0, z: 0.0 };
        let mut translation = Translation { x: -2.0, y: 0.0, z: 0.0 };
        restrict(&shape2, &mut translation, &shape1);
        assert_eq!(translation, Translation { x: -1.0, y: 0.0, z: 0.0 });

        // The other way around
        let shape2 = shape1.clone() + Translation { x: -2.0, y: 0.0, z: 0.0 };
        let mut translation = Translation { x: 2.0, y: 0.0, z: 0.0 };
        restrict(&shape2, &mut translation, &shape1);
        assert_eq!(translation, Translation { x: 1.0, y: 0.0, z: 0.0 });

        // From top
        let shape2 = shape1.clone() + Translation { x: 0.0, y: 2.0, z: 0.0 };
        let mut translation = Translation { x: 0.0, y: -2.0, z: 0.0 };
        restrict(&shape2, &mut translation, &shape1);
        assert_eq!(translation, Translation { x: 0.0, y: -1.0, z: 0.0 });

        // On last axis
        let shape2 = shape1.clone() + Translation { x: 0.0, y: 0.0, z: 2.0 };
        let mut translation = Translation { x: 0.0, y: 0.0, z: -2.0 };
        restrict(&shape2, &mut translation, &shape1);
        assert_eq!(translation, Translation { x: 0.0, y: 0.0, z: -1.0 });

        // Colliding on corner
        let shape2 = shape1.clone() + Translation { x: 2.0, y: 2.0, z: 2.0 };
        let mut translation = Translation { x: -2.0, y: -2.0, z: -2.0 };
        restrict(&shape2, &mut translation, &shape1);
        assert_eq!(translation, Translation { x: -1.0, y: -1.0, z: -1.0 });

        // Colliding with offset on other axis
        let shape2 = shape1.clone() + Translation { x: 2.0, y: 0.5, z: 0.0 };
        let mut translation = Translation { x: -2.0, y: 0.0, z: 0.0 };
        restrict(&shape2, &mut translation, &shape1);
        assert_eq!(translation, Translation { x: -1.0, y: 0.0, z: 0.0 });

        // Colliding when already inside
        let shape2 = shape1.clone() + Translation { x: 0.5, y: 0.5, z: 0.5 };
        let mut translation = Translation { x: -0.5, y: -0.5, z: -0.5 };
        restrict(&shape2, &mut translation, &shape1);
        assert_eq!(translation, Translation { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn test_ray_cast() {
        let shape = CollisionShape {
            x1: 0.0,
            y1: 0.0,
            z1: 0.0,
            x2: 1.0,
            y2: 1.0,
            z2: 1.0,
        };

        let movement = Translation { x: 5.0, y: 0.0, z: 0.0 };
        let mini_movements = ray_cast(shape.clone(), movement);
        println!("{mini_movements:#?}");

        let movement = Translation { x: 4.0, y: 2.0, z: 0.0 };
        let mini_movements = ray_cast(shape.clone(), movement);
        println!("{mini_movements:#?}");

        let movement = Translation { x: 2.38, y: 1.82, z: 1.0 };
        let mini_movements = ray_cast(shape.clone(), movement);
        println!("{mini_movements:#?}");
    }
}
