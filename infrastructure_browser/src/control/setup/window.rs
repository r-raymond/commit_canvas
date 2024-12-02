use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::globals::{PanAndZoom, PAN_AND_ZOOM_STATE, SVG, WINDOW};

pub fn update_viewbox(pan_and_zoom: &PanAndZoom) {
    SVG.with(|s| {
        let view_box = format!(
            "{} {} {} {}",
            pan_and_zoom.pan.0,
            pan_and_zoom.pan.1,
            pan_and_zoom.size.0 * pan_and_zoom.zoom,
            pan_and_zoom.size.1 * pan_and_zoom.zoom
        );
        s.set_attribute("viewBox", &view_box).unwrap();
    });
}

pub fn setup() -> Result<(), JsValue> {
    let on_resize_closure = Closure::<dyn Fn()>::new(|| {
        WINDOW.with(|w| {
            PAN_AND_ZOOM_STATE.with(|p| {
                let mut pan_and_zoom = p.borrow_mut();
                pan_and_zoom.size = (
                    w.inner_width().unwrap().as_f64().unwrap() as f32,
                    w.inner_height().unwrap().as_f64().unwrap() as f32,
                );
                update_viewbox(&pan_and_zoom);
            });
        });
    });
    WINDOW.with(|w| {
        w.set_onresize(Some(on_resize_closure.as_ref().unchecked_ref()));
    });
    on_resize_closure.forget();

    let on_load_closure = Closure::<dyn Fn()>::new(|| {
        WINDOW.with(|w| {
            PAN_AND_ZOOM_STATE.with(|p| {
                let mut pan_and_zoom = p.borrow_mut();
                pan_and_zoom.size = (
                    w.inner_width().unwrap().as_f64().unwrap() as f32,
                    w.inner_height().unwrap().as_f64().unwrap() as f32,
                );
                update_viewbox(&pan_and_zoom);
            });
        });
    });

    WINDOW.with(|w| {
        w.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
    });
    on_load_closure.forget();
    Ok(())
}
