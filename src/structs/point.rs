#[derive(Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, point: &Point) -> f64 {
        (
            (self.x - point.x) * (self.x - point.x) + (self.y - point.y) * (self.y - point.y)
        )
            .sqrt()
    }
}

#[test]
fn test_distance() {
    let point1 = Point {
        x: 0.0,
        y: 0.0,
    };
    let point2 = Point {
        x: 0.0,
        y: -2.0
    };
    let distance = point1.distance_to(&point2);
    assert!(distance.eq(&2.0));
}