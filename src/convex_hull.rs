use crate::Point;

/// Finds the [Convex Hull] for a given set of [`Point`]s in counter clockwise order.
///
/// ## Examples
///
/// ```
/// use myelin_geometry::{ConvexHull, Point};
///
/// let convex_hull = ConvexHull::try_new(&[
///     Point { x: 0.0, y: 0.0 },
///     Point { x: 20.0, y: 0.0 },
///     Point { x: 10.0, y: 5.0 },
///     Point { x: 10.0, y: 10.0 },
/// ])
/// .unwrap();
///
/// let expected_convex_hull_points = &[
///     Point { x: 0.0, y: 0.0 },
///     Point { x: 20.0, y: 0.0 },
///     Point { x: 10.0, y: 10.0 },
/// ];
///
/// let convex_hull_points: Vec<_> = convex_hull.collect();
/// assert_eq!(expected_convex_hull_points, convex_hull_points.as_slice());
/// ```
///
/// [Convex Hull]: http://jeffe.cs.illinois.edu/teaching/373/notes/x05-convexhull.pdf
/// [`Point`]: ./struct.Point.html
#[derive(Debug)]
pub struct ConvexHull<'a> {
    points: &'a [Point],
    leftmost_point: Point,
    current_point: Point,
    state: ConvexHullState,
}

#[derive(Debug)]
enum ConvexHullState {
    Initial,
    FindingNextPoint,
}

impl<'a> ConvexHull<'a> {
    /// Constructs a new [`ConvexHull`] from a given set of points.
    ///
    /// ## Errors
    /// Returns an error when zero points are given.
    ///
    /// [`ConvexHull`]: ./struct.ConvexHull.html
    pub fn try_new(points: &'a [Point]) -> Result<Self, ()> {
        if points.is_empty() {
            Err(())
        } else {
            // Safe unwrap: Points should not be baloney like NaN
            let leftmost_point = *points
                .iter()
                .min_by(|a, b| a.partial_cmp(&b).unwrap())
                .expect("At least one point must be given");
            Ok(Self {
                points,
                leftmost_point,
                current_point: leftmost_point,
                state: ConvexHullState::Initial,
            })
        }
    }
}

impl<'a> Iterator for ConvexHull<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            ConvexHullState::Initial => {
                self.state = ConvexHullState::FindingNextPoint;
                Some(self.leftmost_point)
            }
            ConvexHullState::FindingNextPoint => self.find_next_point(),
        }
    }
}

impl<'a> ConvexHull<'a> {
    /// Implementation of [Jarvis March]
    ///
    /// [Jarvis March]: https://www.algorithm-archive.org/contents/jarvis_march/jarvis_march.html
    fn find_next_point(&mut self) -> Option<Point> {
        let first_point = *self.points.first().unwrap();

        self.current_point = self
            .points
            .iter()
            .skip(1)
            .fold(first_point, |endpoint, &point| {
                if endpoint == self.current_point
                    || !is_counter_clockwise_turn(point, self.current_point, endpoint)
                {
                    point
                } else {
                    endpoint
                }
            });

        if self.leftmost_point == self.current_point {
            None
        } else {
            Some(self.current_point)
        }
    }
}

/// Source: <http://jeffe.cs.illinois.edu/teaching/373/notes/x05-convexhull.pdf> (Page 2)
fn is_counter_clockwise_turn(p1: Point, p2: Point, p3: Point) -> bool {
    (p3.y - p1.y) * (p2.x - p1.x) >= (p2.y - p1.y) * (p3.x - p1.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_fails_with_zero_points() {
        assert!(ConvexHull::try_new(&[]).is_err());
    }

    #[test]
    fn convex_hull_iterator_works() {
        let points = vec![
            Point { x: 5.0, y: 5.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 15.0, y: 0.0 },
            Point { x: 20.0, y: 5.0 },
            Point { x: 10.0, y: 10.0 },
        ];

        let hull: Vec<_> = ConvexHull::try_new(&points).unwrap().collect();

        assert_eq!(points, hull);
    }

    #[test]
    fn convex_hull_iterator_works_when_items_are_not_sorted() {
        let points = vec![
            Point { x: 20.0, y: 5.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 15.0, y: 0.0 },
            Point { x: 5.0, y: 5.0 },
        ];

        let expected_hull = vec![
            Point { x: 5.0, y: 5.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 15.0, y: 0.0 },
            Point { x: 20.0, y: 5.0 },
            Point { x: 10.0, y: 10.0 },
        ];

        let hull: Vec<_> = ConvexHull::try_new(&points).unwrap().collect();

        assert_eq!(expected_hull, hull);
    }

    #[test]
    fn convex_hull_iterator_works_with_concave_points() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 20.0, y: 0.0 },
            Point { x: 10.0, y: 5.0 },
            Point { x: 10.0, y: 10.0 },
        ];

        let expected_hull = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 20.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
        ];

        let hull: Vec<_> = ConvexHull::try_new(&points).unwrap().collect();

        assert_eq!(expected_hull, hull);
    }
}
