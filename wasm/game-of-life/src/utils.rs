/// When the `console_error_panic_hook` feature is enabled, we can call the
/// `set_panic_hook` function at least once during initialization, and then
/// we will get better error messages if our code ever panics.
///
/// For more details see
/// https://github.com/rustwasm/console_error_panic_hook#readme
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use web_sys::console;

/// Return the time elapsed since the [time origin](https://developer.mozilla.org/en-US/docs/Web/API/DOMHighResTimeStamp#The_time_origin),
/// in milliseconds.
pub fn now() -> f64 {
    web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now()
}

/// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! console_log {
    ($( $t:tt )*) => (console::log_1(&format!($( $t )*).into()))
}

/// A struct that calls JavaScript's `console.time` with `label` when it is
/// created, and `console.timeEnd` when it is dropped.
pub struct Timer<'a> {
    label: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(label: &'a str) -> Self {
        console::time_with_label(label);
        Self { label }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.label);
    }
}
