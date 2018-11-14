//! Types relating to 2D convex polygons and their construction

use super::*;
mod builder;
pub use self::builder::*;

/// A convex polygon
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Polygon {
    /// The vertices of the polygon
    pub vertices: Vec<Point>,
}

impl Polygon {
    /// Apply translation specified by `translation`, represented as
    /// a relative point
    pub fn translate(&self, translation: Point) -> Self {
        let translated_vertices = self
            .vertices
            .iter()
            .map(|&vertex| vertex + translation)
            .collect();

        Polygon {
            vertices: translated_vertices,
        }
    }

    /// Rotate polygon by a `rotation` around a `point`
    pub fn rotate_around_point(&self, rotation: Radians, point: Point) -> Self {
        let rotation = rotation.value();
        let rotated_vertices = self
            .vertices
            .iter()
            .map(|&vertex| {
                // See https://en.wikipedia.org/wiki/Rotation_matrix
                let delta = vertex - point;
                let (rotation_sin, rotation_cos) = rotation.sin_cos();
                let rotated_x = rotation_cos * delta.x + rotation_sin * delta.y + point.x;
                let rotated_y = -rotation_sin * delta.x + rotation_cos * delta.y + point.y;
                Point {
                    x: rotated_x,
                    y: rotated_y,
                }
            })
            .collect();
        Self {
            vertices: rotated_vertices,
        }
    }

    /// Checks if a given point rests inside the polygon
    pub fn contains_point(&self, point: Point) -> bool {
        // Anything less than a line cannot "contain" anything
        if self.vertices.len() < 2 {
            return false;
        }

        /// The side that a [`Point`] lies on, from the
        /// point of view of a line
        #[derive(Eq, PartialEq, Debug)]
        enum Side {
            /// The point lies to the right of the line
            Right,
            /// The point lies to the left of the line
            Left,
            /// The point is exactly on the line
            OnTheLine,
        };

        /// Calculate which on which side of a line from `a` to `b` a
        /// given `point` is
        fn calculate_facing_side(a: Vector, b: Vector, point: Vector) -> Side {
            let side_vector = b - a;
            let vector_from_a_to_point = point - a;
            let cross_product = side_vector.cross_product(vector_from_a_to_point);

            /// Minimal distance from `point` to the line to be considered
            /// exactly *on* the line
            const EPSILON: f64 = 0.000001;
            if cross_product < -EPSILON {
                Side::Left
            } else if cross_product > EPSILON {
                Side::Right
            } else {
                Side::OnTheLine
            }
        };

        let vector_to_point: Vector = point.into();
        // The following unwraps are safe, as we do an
        // early return if we don't contain at least 2 vertices
        let vector_to_last_point: Vector = (*self.vertices.last().unwrap()).into();
        let vector_to_first_point: Vector = (*self.vertices.first().unwrap()).into();
        let reference_side =
            calculate_facing_side(vector_to_last_point, vector_to_first_point, vector_to_point);

        // If the point lies on the same side of all lines of the polygon,
        // the point is contained in the polygon.
        // We iterate until len() - 1 because inside the loop, we access i + 1.
        for i in 0..self.vertices.len() - 1 {
            let vector_to_point_a: Vector = self.vertices[i].into();
            let vector_to_point_b: Vector = self.vertices[i + 1].into();
            let side = calculate_facing_side(vector_to_point_a, vector_to_point_b, vector_to_point);
            if side != reference_side && side != Side::OnTheLine {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use self::builder::PolygonBuilder;
    use super::*;
    use std::f64::consts::PI;

    fn polygon() -> Polygon {
        PolygonBuilder::default()
            .vertex(-10.0, -10.0)
            .vertex(10.0, -10.0)
            .vertex(10.0, 10.0)
            .vertex(-10.0, 10.0)
            .build()
            .unwrap()
    }

    fn translation() -> Point {
        Point { x: 30.0, y: 40.0 }
    }

    #[test]
    fn translates() {
        let polygon = polygon();
        assert_eq!(
            Polygon {
                vertices: vec![
                    Point { x: 20.0, y: 30.0 },
                    Point { x: 40.0, y: 30.0 },
                    Point { x: 40.0, y: 50.0 },
                    Point { x: 20.0, y: 50.0 },
                ],
            },
            polygon.translate(translation())
        );
    }

    #[test]
    fn rotates_by_pi() {
        let polygon = polygon();

        const FLOATING_POINT_INACCURACY: f64 = 0.000000000000002;
        assert_eq!(
            Polygon {
                vertices: vec![
                    Point {
                        x: 10.0 - FLOATING_POINT_INACCURACY,
                        y: 10.0 + FLOATING_POINT_INACCURACY
                    },
                    Point {
                        x: -10.0 - FLOATING_POINT_INACCURACY,
                        y: 10.0 - FLOATING_POINT_INACCURACY
                    },
                    Point {
                        x: -10.0 + FLOATING_POINT_INACCURACY,
                        y: -10.0 - FLOATING_POINT_INACCURACY
                    },
                    Point {
                        x: 10.0 + FLOATING_POINT_INACCURACY,
                        y: -10.0 + FLOATING_POINT_INACCURACY
                    },
                ],
            },
            polygon.rotate_around_point(Radians::try_new(PI).unwrap(), Point::default())
        );
    }

    #[test]
    fn rotates_by_arbitrary_orientation() {
        let polygon = polygon();

        const ROTATION_A: f64 = 8.488724885405782;
        const ROTATION_B: f64 = 11.311125046603125;

        assert_eq!(
            Polygon {
                vertices: vec![
                    Point {
                        x: ROTATION_A,
                        y: ROTATION_B
                    },
                    Point {
                        x: -ROTATION_B,
                        y: ROTATION_A
                    },
                    Point {
                        x: -ROTATION_A,
                        y: -ROTATION_B
                    },
                    Point {
                        x: ROTATION_B,
                        y: -ROTATION_A
                    },
                ],
            },
            polygon.rotate_around_point(Radians::try_new(3.0).unwrap(), Point::default())
        );
    }

    #[test]
    fn translates_and_rotates() {
        let polygon = polygon();
        let translation = translation();
        let translated_polygon = polygon.translate(translation);

        assert_eq!(
            Polygon {
                vertices: vec![
                    Point {
                        x: 38.48872488540578,
                        y: 51.311125046603124
                    },
                    Point {
                        x: 18.688874953396876,
                        y: 48.48872488540578
                    },
                    Point {
                        x: 21.51127511459422,
                        y: 28.688874953396876
                    },
                    Point {
                        x: 41.311125046603124,
                        y: 31.51127511459422
                    },
                ],
            },
            translated_polygon.rotate_around_point(Radians::try_new(3.0).unwrap(), translation)
        );
    }

    #[test]
    fn contains_point_when_point_is_positive() {
        let translation = Point { x: 10.43, y: 20.1 };
        let polygon = polygon().translate(translation);
        let point = Point { x: 12.0, y: 18.0 };
        assert!(polygon.contains_point(point));
    }

    #[test]
    fn contains_point_when_point_is_negative() {
        let translation = Point { x: -20.0, y: -5.0 };
        let polygon = polygon().translate(translation);
        let point = Point { x: -21.70, y: -2.3 };
        assert!(polygon.contains_point(point));
    }

    #[test]
    fn contains_point_when_point_is_at_zero() {
        let polygon = polygon();
        let point = Point::default();
        assert!(polygon.contains_point(point));
    }

    #[test]
    fn contains_point_at_border() {
        let polygon = polygon();
        let point = Point { x: 10.0, y: -10.0 };
        assert!(polygon.contains_point(point));
    }

    #[test]
    fn does_not_contain_point_barely_outside_polygon() {
        let polygon = polygon();
        let point = Point { x: 10.1, y: -10.1 };
        assert!(!polygon.contains_point(point));
    }

    #[test]
    fn does_not_contain_point_way_outside_polygon() {
        let polygon = polygon();
        let point = Point {
            x: -9000.0,
            y: -9000.0,
        };
        assert!(!polygon.contains_point(point));
    }

    #[test]
    fn does_not_contain_point_when_point_is_at_zero() {
        let translation = Point { x: 11.0, y: 11.0 };
        let polygon = polygon().translate(translation);
        let point = Point::default();
        assert!(!polygon.contains_point(point));
    }
}
