use super::point::Point;

#[derive(Clone)]
pub struct Ring {
    pub centre: Point,
    pub radius: f64,
}

impl Ring {
    pub fn is_point_in_ring(&self, point: &Point) -> bool {
        self.centre.distance_to(&point) < self.radius
    }
    pub fn from_3_points(p1: &Point, p2: &Point, p3: &Point) -> Self {
        let centre = Point {
            x: -0.5 * (p1.y*(p2.x*p2.x+p2.y*p2.y-p3.x*p3.x-p3.y*p3.y) + p2.y*(p3.x*p3.x+p3.y*p3.y-p1.x*p1.x-p1.y*p1.y) + p3.y*(p1.x*p1.x+p1.y*p1.y-p2.x*p2.x-p2.y*p2.y))
            /
                (p1.x*(p2.y-p3.y) + p2.x*(p3.y-p1.y) + p3.x*(p1.y-p2.y)),
            y: 0.5 * (p1.x*(p2.x*p2.x+p2.y*p2.y-p3.x*p3.x-p3.y*p3.y) + p2.x*(p3.x*p3.x+p3.y*p3.y-p1.x*p1.x-p1.y*p1.y) + p3.x*(p1.x*p1.x+p1.y*p1.y-p2.x*p2.x-p2.y*p2.y))
                /
                (p1.x*(p2.y-p3.y) + p2.x*(p3.y-p1.y) + p3.x*(p1.y-p2.y)),
        };
        let radius = centre.distance_to(&p1);
        Self {
            centre,
            radius
        }
    }
}

#[test]
fn test_point_in_ring() {
    let ring = Ring {
        centre: Point {
            x: 1.0,
            y: 2.0
        },
        radius: 5.0,
    };
    let point = Point {
        x: 3.0,
        y: 3.0,
    };
    let is_point_in_ring = ring.is_point_in_ring(&point);
    assert_eq!(is_point_in_ring, true);
}

#[test]
fn test_ring_triangle() {
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
    let ring = Ring::from_3_points(&p1, &p2, &p3);
    assert_eq!(ring.centre.x, 0.0);
    assert_eq!(ring.centre.y, 0.0);
    assert_eq!(ring.radius, 1.0);
}