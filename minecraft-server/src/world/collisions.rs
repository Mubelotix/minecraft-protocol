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

#[derive(Debug, Clone, PartialEq)]
pub struct BlockRange {
    x: std::ops::Range<i32>,
    y: std::ops::Range<i32>,
    z: std::ops::Range<i32>,
}

#[derive(Debug, Clone)]
pub struct ExcludingBlockRange {
    range: BlockRange,
    exclusion: BlockRange,
}

impl IntoIterator for BlockRange {
    type Item = BlockPosition;
    type IntoIter = BlockRangeIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        BlockRangeIntoIter {
            x: self.x.start,
            y: self.y.start,
            z: self.z.start,
            range: self,
        }
    }
}

impl IntoIterator for ExcludingBlockRange {
    type Item = BlockPosition;
    type IntoIter = ExcludingBlockRangeIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        ExcludingBlockRangeIntoIter {
            x: self.range.x.start,
            y: self.range.y.start,
            z: self.range.z.start,
            range: self,
        }
    }
}

pub struct BlockRangeIntoIter {
    range: BlockRange,
    x: i32,
    y: i32,
    z: i32,
}

pub struct ExcludingBlockRangeIntoIter {
    range: ExcludingBlockRange,
    x: i32,
    y: i32,
    z: i32,
}

impl Iterator for BlockRangeIntoIter {
    type Item = BlockPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.range.x.end {
            self.x = self.range.x.start;
            self.y += 1;
            if self.y >= self.range.y.end {
                self.y = self.range.y.start;
                self.z += 1;
                if self.z >= self.range.z.end {
                    return None;
                }
            }
        }
        let r = BlockPosition {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        self.x += 1;
        Some(r)
    }
}

impl Iterator for ExcludingBlockRangeIntoIter {
    type Item = BlockPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.range.range.x.end {
            self.x = self.range.range.x.start;
            self.y += 1;
            if self.y >= self.range.range.y.end {
                self.y = self.range.range.y.start;
                self.z += 1;
                if self.z >= self.range.range.z.end {
                    return None;
                }
            }
        }
        let r = BlockPosition {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        self.x += 1;
        if self.range.exclusion.x.contains(&r.x) && self.range.exclusion.y.contains(&r.y) && self.range.exclusion.z.contains(&r.z) {
            return self.next();
        }
        Some(r)
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
    pub fn containing_blocks(&self) -> BlockRange {
        BlockRange {
            x: (self.x1.floor() as i32)..(self.x2.ceil() as i32),
            y: (self.y1.floor() as i32)..(self.y2.ceil() as i32),
            z: (self.z1.floor() as i32)..(self.z2.ceil() as i32),
        }
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Translation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Translation {
    pub fn zero() -> Self {
        Self::default()
    }

    pub fn yaw_pitch(&self) -> (f32, f32) {
        let r = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let pitch = -(self.y / r).asin() / std::f64::consts::PI * 180.0;
        let yaw = -(self.x / self.z).atan() / std::f64::consts::PI * 180.0;
        (yaw as f32, pitch as f32)
    }
}

pub struct TranslationFragmentIterator<'a> {
    translation: &'a Translation,
    position: &'a CollisionShape,
    fragmented: Translation,
    previous_block_range: Option<BlockRange>,
}

impl<'a> Iterator for TranslationFragmentIterator<'a> {
    type Item = (Translation, ExcludingBlockRange);

    fn next(&mut self) -> Option<Self::Item> {
        if self.previous_block_range.is_none() {
            let block_range = self.position.containing_blocks();
            self.previous_block_range = Some(block_range.clone());
            return Some((self.fragmented.clone(), ExcludingBlockRange {
                range: block_range,
                exclusion: BlockRange {
                    x: 0..0,
                    y: 0..0,
                    z: 0..0,
                },
            }));
        }
        let mut mini_translation = if self.fragmented.norm() < self.translation.norm() {
            let x_dist = if self.translation.x > 0.0 {
                let next_x = (self.position.x2 + self.fragmented.x).floor()+1.0;
                (next_x - (self.position.x2 + self.fragmented.x)).abs()
            } else {
                let next_x = (self.position.x1 + self.fragmented.x).ceil()-1.0;
                (next_x - (self.position.x1 + self.fragmented.x)).abs()
            };
            let y_dist = if self.translation.y > 0.0 {
                let next_y = (self.position.y2 + self.fragmented.y).floor()+1.0;
                (next_y - (self.position.y2 + self.fragmented.y)).abs()
            } else {
                let next_y = (self.position.y1 + self.fragmented.y).ceil()-1.0;
                (next_y - (self.position.y1 + self.fragmented.y)).abs()
            };
            let z_dist = if self.translation.z > 0.0 {
                let next_z = (self.position.z2 + self.fragmented.z).floor()+1.0;
                (next_z - (self.position.z2 + self.fragmented.z)).abs()
            } else {
                let next_z = (self.position.z1 + self.fragmented.z).ceil()-1.0;
                (next_z - (self.position.z1 + self.fragmented.z)).abs()
            };
            let x_time = x_dist / self.translation.x.abs();
            let y_time = y_dist / self.translation.y.abs();
            let z_time = z_dist / self.translation.z.abs();
            let time = min(x_time, y_time, z_time);
            //println!("pos{fragmented:?} dist({x_dist}, {y_dist}, {z_dist}) time({x_time}, {y_time}, {z_time}) time({time})");
            let mini_translation = self.translation.clone() * time;
            self.fragmented += &mini_translation;
            mini_translation
        } else {
            return None;
        };
        if self.fragmented.norm() >= self.translation.norm() {
            let final_position = self.position.clone() + self.translation;
            let previous_fragmented = self.fragmented.clone() - mini_translation;
            let previous_position = self.position.clone() + previous_fragmented;
            let difference = Translation {
                x: final_position.x1 - previous_position.x1,
                y: final_position.y1 - previous_position.y1,
                z: final_position.z1 - previous_position.z1,
            };
            mini_translation = difference;
        }
        let current_position = self.position.clone() + &self.fragmented;
        let block_range = current_position.containing_blocks();
        let previous_block_range = self.previous_block_range.take().unwrap_or(BlockRange {
            x: 0..0,
            y: 0..0,
            z: 0..0,
        });
        let excluding_block_range = ExcludingBlockRange {
            range: block_range.clone(),
            exclusion: previous_block_range,
        };
        self.previous_block_range = Some(block_range);
        Some((mini_translation, excluding_block_range))
    }
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

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Keep direction but change norm
    pub fn set_norm(&mut self, norm: f64) {
        let current_norm = self.norm();
        if current_norm == 0.0 {
            return;
        }
        self.x *= norm / current_norm;
        self.y *= norm / current_norm;
        self.z *= norm / current_norm;
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn fragment<'a>(&'a self, position: &'a CollisionShape) -> TranslationFragmentIterator<'a> {
        TranslationFragmentIterator {
            translation: self,
            position,
            fragmented: Translation { x: 0.0, y: 0.0, z: 0.0 },
            previous_block_range: None,
        }
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

impl std::ops::AddAssign<Translation> for Position {
    fn add_assign(&mut self, rhs: Translation) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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
    fn test_containing_blocks() {
        let shape = CollisionShape {
            x1: 0.0,
            y1: 0.0,
            z1: 0.0,
            x2: 1.0,
            y2: 1.0,
            z2: 1.0,
        };
        assert_eq!(shape.containing_blocks(), BlockRange { x: 0..1, y: 0..1, z: 0..1 });
        assert_eq!(shape.containing_blocks().into_iter().collect::<Vec<_>>(), vec![BlockPosition {x: 0, y: 0, z: 0}]);

        let shape = CollisionShape {
            x1: 0.0,
            y1: 0.0,
            z1: 0.0,
            x2: 1.1,
            y2: 1.0,
            z2: 1.0,
        };
        assert_eq!(shape.containing_blocks(), BlockRange { x: 0..2, y: 0..1, z: 0..1 });
        assert_eq!(shape.containing_blocks().into_iter().collect::<Vec<_>>(), vec![BlockPosition {x: 0, y: 0, z: 0}, BlockPosition {x: 1, y: 0, z: 0}]);
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
        assert_eq!(
            fragments.map(|(t,b)| (t, b.into_iter().collect::<Vec<_>>())).collect::<Vec<_>>(),
            vec![
                (Translation { x: 0.0, y: 0.0, z: 0.0 }, vec![BlockPosition { x: 0, y: 0, z: 0 }]),
                (Translation { x: 1.0, y: 0.0, z: 0.0 }, vec![BlockPosition { x: 1, y: 0, z: 0 }]),
                (Translation { x: 1.0, y: 0.0, z: 0.0 }, vec![BlockPosition { x: 2, y: 0, z: 0 }]),
                (Translation { x: 1.0, y: 0.0, z: 0.0 }, vec![BlockPosition { x: 3, y: 0, z: 0 }])
            ]
        );

        let movement = Translation { x: 2.3, y: 0.0, z: 0.0 };
        let fragments = movement.fragment(&shape);
        assert_eq!(fragments.map(|(t,_)| t).collect::<Vec<Translation>>(), vec![
            Translation { x: 0.0, y: 0.0, z: 0.0 },
            Translation { x: 1.0, y: 0.0, z: 0.0 },
            Translation { x: 1.0, y: 0.0, z: 0.0 },
            Translation { x: 0.2999999999999998, y: 0.0, z: 0.0 }
        ]);

        let movement = Translation { x: 1.0, y: 0.75, z: 0.0 } * 4.0;
        let fragments = movement.fragment(&shape);
        assert_eq!(fragments.map(|(t,_)| t).collect::<Vec<Translation>>(), vec![
            Translation { x: 0.0, y: 0.0, z: 0.0 },
            Translation { x: 1.0, y: 0.75, z: 0.0 },
            Translation { x: 0.3333333333333333, y: 0.25, z: 0.0 },
            Translation { x: 0.666666666666667, y: 0.5000000000000002, z: 0.0 },
            Translation { x: 0.6666666666666666, y: 0.5, z: 0.0 },
            Translation { x: 0.3333333333333335, y: 0.2500000000000001, z: 0.0 },
            Translation { x: 1.0, y: 0.75, z: 0.0 }]
        );
    }
}
