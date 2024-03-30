pub type Float = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

impl From<(Float, Float)> for Point {
    fn from((x, y): (Float, Float)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (Float, Float) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}
