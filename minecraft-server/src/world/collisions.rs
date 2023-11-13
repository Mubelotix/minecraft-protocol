use super::*;

/// Returns minimum of two floats, but not NaN
fn min2(a: f64, b: f64) -> f64 {
    if a < b || b.is_nan() {
        a
    } else {
        b
    }
}

/// Returns minimum of three floats
fn min(a: f64, b: f64, c: f64) -> f64 {
    min2(min2(a, b), c)
}

/// An object in space
#[derive(Debug, Clone, PartialEq)]
pub struct CollisionShape {
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub x2: f64,
    pub y2: f64,
    pub z2: f64,
}

impl CollisionShape {
    const fn points(&self) -> PointIter {
        PointIter {
            shape: self,
            index: 0,
        }
    }

    // TODO(perf): Return an iterator yielding blocks instead of a vec of blocks
    pub fn containing_blocks(&self) -> Vec<BlockPosition> {
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

/// A point in space
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    /// Returns true if the point is inside the shape
    fn is_inside(&self, shape: &CollisionShape) -> bool {
        (shape.x1..=shape.x2).contains(&self.x) && (shape.y1..=shape.y2).contains(&self.y) && (shape.z1..=shape.z2).contains(&self.z)
    }

    /// Returns the proportion of the translation that can be applied without absorbing `point` inside `shape` on the x axis
    fn collide_x(&self, shape: &CollisionShape, translation: &Translation) -> f64 {
        if translation.x == 0.0 {
            return 1.0;
        }
        let x = if translation.x < 0.0 { shape.x1 } else { shape.x2 };
        let translated_ratio = (self.x - x) / translation.x;
        if translated_ratio >= 1.0 {
            return 1.0;
        } else if translated_ratio <= 0.0 {
            return 0.0;
        }
        let translated_y1 = shape.y1 + translation.y * translated_ratio;
        let translated_y2 = shape.y2 + translation.y * translated_ratio;
        let translated_z1 = shape.z1 + translation.z * translated_ratio;
        let translated_z2 = shape.z2 + translation.z * translated_ratio;
        if (translated_y1..=translated_y2).contains(&self.y) && (translated_z1..=translated_z2).contains(&self.z) {
            translated_ratio
        } else {
            1.0
        }
    }

    /// Returns the proportion of the translation that can be applied without absorbing `point` inside `shape` on the y axis
    fn collide_y(&self, shape: &CollisionShape, translation: &Translation) -> f64 {
        if translation.y == 0.0 {
            return 1.0;
        }
        let y = if translation.y < 0.0 { shape.y1 } else { shape.y2 };
        let translated_ratio = (self.y - y) / translation.y;
        if translated_ratio >= 1.0 {
            return 1.0;
        } else if translated_ratio <= 0.0 {
            return 0.0;
        }
        let translated_x1 = shape.x1 + translation.x * translated_ratio;
        let translated_x2 = shape.x2 + translation.x * translated_ratio;
        let translated_z1 = shape.z1 + translation.z * translated_ratio;
        let translated_z2 = shape.z2 + translation.z * translated_ratio;
        if (translated_x1..=translated_x2).contains(&self.x) && (translated_z1..=translated_z2).contains(&self.z) {
            translated_ratio
        } else {
            1.0
        }
    }

    /// Returns the proportion of the translation that can be applied without absorbing `point` inside `shape` on the z axis
    fn collide_z(&self, shape: &CollisionShape, translation: &Translation) -> f64 {
        if translation.z == 0.0 {
            return 1.0;
        }
        let z = if translation.z < 0.0 { shape.z1 } else { shape.z2 };
        let translated_ratio = (self.z - z) / translation.z;
        if translated_ratio >= 1.0 {
            return 1.0;
        } else if translated_ratio <= 0.0 {
            return 0.0;
        }
        let translated_x1 = shape.x1 + translation.x * translated_ratio;
        let translated_x2 = shape.x2 + translation.x * translated_ratio;
        let translated_y1 = shape.y1 + translation.y * translated_ratio;
        let translated_y2 = shape.y2 + translation.y * translated_ratio;
        if (translated_x1..=translated_x2).contains(&self.x) && (translated_y1..=translated_y2).contains(&self.y) {
            translated_ratio
        } else {
            1.0
        }
    }

    /// Returns the proportion of the translation that can be applied without absorbing `point` inside `shape`
    fn collide(&self, shape: &CollisionShape, translation: &Translation) -> f64 {
        min(
            self.collide_x(shape, translation),
            self.collide_y(shape, translation),
            self.collide_z(shape, translation)
        )
    }
}

/// An iterator over the 8 corners of a [CollisionShape]
pub struct PointIter<'a> {
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

/// Vector describing a movement
#[derive(Debug, Clone, PartialEq)]
pub struct Translation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Translation {
    /// Cuts the translation just enough so that the shape doesn't collide with the obstacle
    fn prevent_collision(&mut self, object: &CollisionShape, obstacle: &CollisionShape) {
        let mut limit = 1.0;

        for point in obstacle.points() {
            limit = min2(limit, point.collide(object, self));
            if limit == 0.0 {
                break;
            }
        }

        self.x *= limit;
        self.y *= limit;
        self.z *= limit;
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // TODO: turn CollisionShape.fragment into an iterator
    pub fn fragment(self, position: &CollisionShape) -> Vec<Translation> {
        let mut result = Vec::new();
        let mut fragmented = Translation { x: 0.0, y: 0.0, z: 0.0 };
        //let mut current_position = position.clone();
        //result.extend(position.containing_blocks().into_iter());
        while fragmented.norm() < self.norm() {
            let x_dist = if self.x > 0.0 {
                let next_x = (position.x2 + fragmented.x).floor()+1.0;
                (next_x - (position.x2 + fragmented.x)).abs()
            } else {
                let next_x = (position.x1 + fragmented.x).ceil()-1.0;
                (next_x - (position.x1 + fragmented.x)).abs()
            };
            let y_dist = if self.y > 0.0 {
                let next_y = (position.y2 + fragmented.y).floor()+1.0;
                (next_y - (position.y2 + fragmented.y)).abs()
            } else {
                let next_y = (position.y1 + fragmented.y).ceil()-1.0;
                (next_y - (position.y1 + fragmented.y)).abs()
            };
            let z_dist = if self.z > 0.0 {
                let next_z = (position.z2 + fragmented.z).floor()+1.0;
                (next_z - (position.z2 + fragmented.z)).abs()
            } else {
                let next_z = (position.z1 + fragmented.z).ceil()-1.0;
                (next_z - (position.z1 + fragmented.z)).abs()
            };
            let x_time = x_dist / self.x.abs();
            let y_time = y_dist / self.y.abs();
            let z_time = z_dist / self.z.abs();
            let time = min(x_time, y_time, z_time);
            //println!("pos{fragmented:?} dist({x_dist}, {y_dist}, {z_dist}) time({x_time}, {y_time}, {z_time}) time({time})");
            let mini_translation = self.clone() * time;
            fragmented += &mini_translation;
            result.push(mini_translation);
        }
        // Last one might be too long
        if let Some(last) = result.pop() {
            let final_position = position.clone() + self;
            let previous_fragmented = fragmented.clone() - last;
            let previous_position = position.clone() + previous_fragmented;
            let difference = Translation {
                x: final_position.x1 - previous_position.x1,
                y: final_position.y1 - previous_position.y1,
                z: final_position.z1 - previous_position.z1,
            };
            result.push(difference);
        }
        result
    }    
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

impl std::ops::AddAssign<&Translation> for Translation {
    fn add_assign(&mut self, rhs: &Translation) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Translation> for Translation {
    type Output = Translation;

    fn sub(self, rhs: Translation) -> Self::Output {
        Translation {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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

impl std::ops::Mul<f64> for Translation {
    type Output = Translation;

    fn mul(self, rhs: f64) -> Self::Output {
        Translation {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
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
        translation.prevent_collision(&shape2, &shape1);
        assert_eq!(translation, Translation { x: 0.0, y: 0.0, z: 0.0 });

        // Boxes are one block away but one comes and pushes the other
        let shape2 = shape1.clone() + Translation { x: 2.0, y: 0.0, z: 0.0 };
        let mut translation = Translation { x: -2.0, y: 0.0, z: 0.0 };
        translation.prevent_collision(&shape2, &shape1);
        assert_eq!(translation, Translation { x: -1.0, y: 0.0, z: 0.0 });

        // The other way around
        let shape2 = shape1.clone() + Translation { x: -2.0, y: 0.0, z: 0.0 };
        let mut translation = Translation { x: 2.0, y: 0.0, z: 0.0 };
        translation.prevent_collision(&shape2, &shape1);
        assert_eq!(translation, Translation { x: 1.0, y: 0.0, z: 0.0 });

        // From top
        let shape2 = shape1.clone() + Translation { x: 0.0, y: 2.0, z: 0.0 };
        let mut translation = Translation { x: 0.0, y: -2.0, z: 0.0 };
        translation.prevent_collision(&shape2, &shape1);
        assert_eq!(translation, Translation { x: 0.0, y: -1.0, z: 0.0 });

        // On last axis
        let shape2 = shape1.clone() + Translation { x: 0.0, y: 0.0, z: 2.0 };
        let mut translation = Translation { x: 0.0, y: 0.0, z: -2.0 };
        translation.prevent_collision(&shape2, &shape1);
        assert_eq!(translation, Translation { x: 0.0, y: 0.0, z: -1.0 });

        // Colliding on corner
        let shape2 = shape1.clone() + Translation { x: 2.0, y: 2.0, z: 2.0 };
        let mut translation = Translation { x: -2.0, y: -2.0, z: -2.0 };
        translation.prevent_collision(&shape2, &shape1);
        assert_eq!(translation, Translation { x: -1.0, y: -1.0, z: -1.0 });

        // Colliding with offset on other axis
        let shape2 = shape1.clone() + Translation { x: 2.0, y: 0.5, z: 0.0 };
        let mut translation = Translation { x: -2.0, y: 0.0, z: 0.0 };
        translation.prevent_collision(&shape2, &shape1);
        assert_eq!(translation, Translation { x: -1.0, y: 0.0, z: 0.0 });

        // Colliding when already inside
        let shape2 = shape1.clone() + Translation { x: 0.5, y: 0.5, z: 0.5 };
        let mut translation = Translation { x: -0.5, y: -0.5, z: -0.5 };
        translation.prevent_collision(&shape2, &shape1);
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

        let movement = Translation { x: 3.0, y: 0.0, z: 0.0 };
        let fragments = movement.fragment(&shape);
        assert_eq!(fragments, vec![Translation { x: 1.0, y: 0.0, z: 0.0 }; 3]);

        let movement = Translation { x: 2.3, y: 0.0, z: 0.0 };
        let fragments = movement.fragment(&shape);
        assert_eq!(fragments, vec![Translation { x: 1.0, y: 0.0, z: 0.0 }, Translation { x: 1.0, y: 0.0, z: 0.0 }, Translation { x: 0.2999999999999998, y: 0.0, z: 0.0 }]);

        let movement = Translation { x: 1.0, y: 0.75, z: 0.0 } * 4.0;
        let fragments = movement.fragment(&shape);
        assert_eq!(fragments, vec![
            Translation { x: 1.0, y: 0.75, z: 0.0 },
            Translation { x: 0.3333333333333333, y: 0.25, z: 0.0 },
            Translation { x: 0.666666666666667, y: 0.5000000000000002, z: 0.0 },
            Translation { x: 0.6666666666666666, y: 0.5, z: 0.0 },
            Translation { x: 0.3333333333333335, y: 0.2500000000000001, z: 0.0 },
            Translation { x: 1.0, y: 0.75, z: 0.0 }]
        );
    }
}
