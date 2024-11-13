pub type Float = f32;
pub type Int = i32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub type PointPixel = Point<Float>;
pub type PointGrid = Point<Int>;

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(point: Point<T>) -> Self {
        (point.x, point.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point: Point<f32> = Point { x: 1.0, y: 2.0 };
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }

    #[test]
    fn test_point_from_tuple() {
        let point: Point<f32> = (1.0, 2.0).into();
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }

    #[test]
    fn test_tuple_from_point() {
        let point: Point<f32> = Point { x: 1.0, y: 2.0 };
        let tuple: (f32, f32) = point.into();
        assert_eq!(tuple, (1.0, 2.0));
    }

    #[test]
    fn test_point_equality() {
        let point1: Point<f32> = Point { x: 1.0, y: 2.0 };
        let point2: Point<f32> = Point { x: 1.0, y: 2.0 };
        assert_eq!(point1, point2);
    }

    #[test]
    fn test_point_inequality() {
        let point1: Point<f32> = Point { x: 1.0, y: 2.0 };
        let point2: Point<f32> = Point { x: 2.0, y: 3.0 };
        assert_ne!(point1, point2);
    }
}
