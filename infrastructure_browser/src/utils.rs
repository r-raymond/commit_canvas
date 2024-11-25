use std::error::Error;
use std::fmt;

use wasm_bindgen::JsValue;

pub fn set_panic_hook() {
    // When the `debug` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();
}

#[derive(Debug)]
struct JsError {
    message: String,
}

impl JsError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for JsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JavaScript error: {}", self.message)
    }
}

impl Error for JsError {}

pub fn to_error(e: JsValue) -> Box<dyn Error + Send + Sync> {
    Box::new(JsError::new(
        e.as_string().unwrap_or("Unknown error".to_string()),
    ))
}
