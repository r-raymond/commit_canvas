pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn length(&self) -> f64 {
        let x = self.end.x as f64 - self.start.x as f64;
        let y = self.end.y as f64 - self.start.y as f64;
        (x * x + y * y).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_length() {
        let line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 3, y: 4 },
        };
        assert_eq!(line.length(), 5.0);
    }
}
