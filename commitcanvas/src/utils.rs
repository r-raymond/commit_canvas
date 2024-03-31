use crate::settings::PIXEL_STEP;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn pixels_to_coords((x, y): (f32, f32)) -> (i32, i32) {
    (
        ((x - PIXEL_STEP) / (2. * PIXEL_STEP)).round() as i32,
        ((y - PIXEL_STEP) / (2. * PIXEL_STEP)).round() as i32,
    )
}

pub fn coords_to_pixels((x, y): (i32, i32)) -> (f32, f32) {
    (
        x as f32 * 2. * PIXEL_STEP + PIXEL_STEP,
        y as f32 * 2. * PIXEL_STEP + PIXEL_STEP,
    )
}
