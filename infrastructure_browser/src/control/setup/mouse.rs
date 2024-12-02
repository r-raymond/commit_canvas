use commitcanvas::control::MouseButton;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::globals::{CONTROL, DOCUMENT, PAN_AND_ZOOM_STATE, SVG};

use super::window::update_viewbox;

pub fn setup() -> Result<(), JsValue> {
    let mouse_update_closure =
        Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            CONTROL.with(|c| {
                let mut control = c.borrow_mut();
                PAN_AND_ZOOM_STATE.with(|p| {
                    let pan_and_zoom = p.borrow();
                    control.mouse_update((
                        event.offset_x() as f32 + pan_and_zoom.pan.0,
                        event.offset_y() as f32 + pan_and_zoom.pan.1,
                    ));
                });
            });
        });
    DOCUMENT.with(|d| d.set_onmousemove(Some(mouse_update_closure.as_ref().unchecked_ref())));
    mouse_update_closure.forget();

    let mouse_down_closure =
        Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            CONTROL.with(|c| {
                let mut control = c.borrow_mut();
                let button = MouseButton::try_from(event.button());

                match button {
                    Ok(button) => control.mouse_down(button),
                    Err(_) => log::error!("Failed to convert mouse button: {}", event.button()),
                }
            });
        });
    SVG.with(|s| s.set_onmousedown(Some(mouse_down_closure.as_ref().unchecked_ref())));
    mouse_down_closure.forget();

    let mouse_up_closure =
        Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            CONTROL.with(|c| {
                let mut control = c.borrow_mut();
                control.mouse_up();
            });
        });
    SVG.with(|s| s.set_onmouseup(Some(mouse_up_closure.as_ref().unchecked_ref())));
    mouse_up_closure.forget();

    let mouse_wheel_closure =
        Closure::<dyn Fn(web_sys::WheelEvent)>::new(move |event: web_sys::WheelEvent| {
            if event.delta_mode() == web_sys::WheelEvent::DOM_DELTA_PIXEL {
                PAN_AND_ZOOM_STATE.with(|p| {
                    let mut pan_and_zoom = p.borrow_mut();
                    pan_and_zoom.pan.0 -= event.delta_x() as f32;
                    pan_and_zoom.pan.1 -= event.delta_y() as f32;
                    update_viewbox(&pan_and_zoom);
                });
            }
        });
    SVG.with(|s| s.set_onwheel(Some(mouse_wheel_closure.as_ref().unchecked_ref())));
    mouse_wheel_closure.forget();

    Ok(())
}
