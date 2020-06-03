mod structs;
use structs::point::Point;
use structs::ring::Ring;

pub fn hello() {
    let point = Point {
        x: 1.1,
        y: 1.3
    };
    println!("hello {}", point.y);
}

pub fn min_disc(points: &[Point]) -> Ring {
    let mut current_ring = smallest_ring_points(&points[0], &points[1]);
    for (i, point) in points.iter().enumerate().skip(2) {
        if !current_ring.is_point_in_ring(&point) {
            current_ring = min_disc_with_point(&points[..i-1], point);
        }
    }
    current_ring
}

fn min_disc_with_point(points: &[Point], q: &Point) -> Ring {
    let mut current_ring = smallest_ring_points(&points[0], &q);
    for (i, point) in points.iter().enumerate().skip(1) {
        if !current_ring.is_point_in_ring(&point) {
            current_ring = min_disc_with_2_points(&points[..i-1], &point, &q);
        }
    }
    current_ring
}

fn min_disc_with_2_points(points: &[Point], q1: &Point, q2: &Point) -> Ring {
    let mut current_ring = smallest_ring_points(&q1, &q2);
    for point in points.iter() {
        if !current_ring.is_point_in_ring(&point) {
            current_ring = Ring::from_3_points(&point, &q1, &q2);
        }
    }
    current_ring
}

fn smallest_ring_points(point1: &Point, point2: &Point) -> Ring {
    Ring {
        centre: Point {
            x: (point1.x + point2.x) / 2.0,
            y: (point1.y + point2.y) / 2.0,
        },
        radius: point1.distance_to(&point2) / 2.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use rand::seq::SliceRandom;

    #[test]
    fn test_3_points() {
        let p1 = Point {
            x: -1.0,
            y: 0.0,
        };
        let p2 = Point {
            x: 0.0,
            y: 1.0,
        };
        let p3 = Point {
            x: 1.0,
            y: 0.0,
        };
        let points = vec![p1, p2, p3];
        let ring = min_disc(&points);
        assert_eq!(ring.centre.x, 0.0);
        assert_eq!(ring.centre.y, 0.0);
        assert_eq!(ring.radius, 1.0);
    }

    #[test]
    fn test_is_circle_always_same() {
        let mut rng = thread_rng();
        let mut points: Vec<Point> = (0..10000).map(|_| {
            Point {
                x: rng.gen(),
                y: rng.gen(),
            }
        }).collect();
        let ring1 = min_disc(&points);
        let mut points_clone = points.clone();
        points_clone.shuffle(&mut rng);
        assert_ne!(points_clone[0], points[0]);
        let ring2 = min_disc(&points_clone);
        assert_eq!(ring2, ring1);
    }

    #[test]
    fn test_all_points_inside_ring() {
        let mut rng = thread_rng();
        let mut points: Vec<Point> = (0..10000).map(|_| {
            Point {
                x: rng.gen(),
                y: rng.gen(),
            }
        }).collect();
        let ring = min_disc(&points);
        points.iter().for_each(|point| { assert!(ring.is_point_in_ring(point)) });
    }
}