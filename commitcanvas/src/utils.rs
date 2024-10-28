use crate::types::{PointGrid, PointPixel};

use crate::settings::PIXEL_STEP;

/// Converts a point in pixel coordinates to grid coordinates.
///
/// # Arguments
///
/// * `PointPixel { x, y }` - A point in pixel coordinates.
///
/// # Returns
///
/// * `PointGrid` - The corresponding point in grid coordinates.
pub fn pixels_to_coords(PointPixel { x, y }: PointPixel) -> PointGrid {
    PointGrid {
        x: ((x - PIXEL_STEP) / (2. * PIXEL_STEP)).round() as i32,
        y: ((y - PIXEL_STEP) / (2. * PIXEL_STEP)).round() as i32,
    }
}

/// Converts a point in grid coordinates to pixel coordinates.
///
/// # Arguments
///
/// * `PointGrid { x, y }` - A point in grid coordinates.
///
/// # Returns
///
/// * `PointPixel` - The corresponding point in pixel coordinates.
pub fn coords_to_pixels(PointGrid { x, y }: PointGrid) -> PointPixel {
    PointPixel {
        x: x as f32 * 2. * PIXEL_STEP + PIXEL_STEP,
        y: y as f32 * 2. * PIXEL_STEP + PIXEL_STEP,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::PIXEL_STEP;
    use crate::types::{PointGrid, PointPixel};

    #[test]
    fn test_pixels_to_coords() {
        let pixel_point = PointPixel { x: 10.0, y: 20.0 };
        let expected_grid_point = PointGrid {
            x: ((10.0 - PIXEL_STEP) / (2. * PIXEL_STEP)).round() as i32,
            y: ((20.0 - PIXEL_STEP) / (2. * PIXEL_STEP)).round() as i32,
        };
        let grid_point = pixels_to_coords(pixel_point);
        assert_eq!(grid_point, expected_grid_point);
    }

    #[test]
    fn test_coords_to_pixels() {
        let grid_point = PointGrid { x: 3, y: 4 };
        let expected_pixel_point = PointPixel {
            x: 3 as f32 * 2. * PIXEL_STEP + PIXEL_STEP,
            y: 4 as f32 * 2. * PIXEL_STEP + PIXEL_STEP,
        };
        let pixel_point = coords_to_pixels(grid_point);
        assert_eq!(pixel_point, expected_pixel_point);
    }
}
