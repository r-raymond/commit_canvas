use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add<&Vector> for Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Point {
        Point {
            x: self.x + rhs.x as i32,
            y: self.y + rhs.y as i32,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Vector {
        Vector {
            x: self.x as f32 - rhs.x as f32,
            y: self.y as f32 - rhs.y as f32,
        }
    }
}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }

    pub fn from_points(start: &Point, end: &Point) -> Vector {
        Vector {
            x: end.x as f32 - start.x as f32,
            y: end.y as f32 - start.y as f32,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length,
        }
    }
}

impl Mul<&Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    pub fn length(&self) -> f32 {
        let x = self.end.x as f32 - self.start.x as f32;
        let y = self.end.y as f32 - self.start.y as f32;
        (x * x + y * y).sqrt()
    }

    pub fn to_catmull_rom_spline(&self, roughness: f32) -> [Point; 4] {
        let length = self.length();
        let mut rng =
            SmallRng::seed_from_u64((self.start.x + self.start.y + self.end.x + self.end.y) as u64);
        let roughness = f32::clamp(0.1 * roughness * length, 0.1, 100.0);
        let r1 = f32::sqrt(rng.gen_range(0.0..roughness));
        let r2 = f32::sqrt(rng.gen_range(0.0..roughness));
        let phi1 = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let phi2 = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let mid_point_offset = 0.005 * length * rng.gen::<f32>();
        let dis_along = rng.gen_range(-0.1..0.1);
        let dis_orth = rng.gen_range(-0.1..0.1) * roughness;

        let start = Point {
            x: (self.start.x as f32 + r1 * f32::cos(phi1)) as i32,
            y: (self.start.y as f32 + r1 * f32::sin(phi1)) as i32,
        };
        let end = Point {
            x: (self.end.x as f32 + r2 * f32::cos(phi2)) as i32,
            y: (self.end.y as f32 + r2 * f32::sin(phi2)) as i32,
        };

        let along = Vector::from_points(&start, &end).normalize();
        let orth = Vector::new(-along.y, along.x);

        let mid_point = self.start + &(0.5 * length * &along) + &(mid_point_offset * &orth);
        let control_point =
            self.start + &((0.75 + dis_along) * length * &along) + &(dis_orth * &orth);

        return [start, mid_point, control_point, end];
    }

    pub fn to_svg_path(&self, roughness: f32) -> String {
        let spline = self.to_catmull_rom_spline(roughness);

        return format!(
            "M {} {} C {} {} {} {}, {} {}",
            spline[0].x,
            spline[0].y,
            spline[1].x,
            spline[1].y,
            spline[2].x,
            spline[2].y,
            spline[3].x,
            spline[3].y
        );
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
