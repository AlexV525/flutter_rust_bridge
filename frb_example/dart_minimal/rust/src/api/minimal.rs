use flutter_rust_bridge::frb;
pub use std::panic::{RefUnwindSafe, UnwindSafe};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO only temp example
#[frb(opaque)]
pub struct Apple {}

impl Apple {
    // #[frb(rust_opaque_codec_moi)]
    pub fn new() -> Apple {
        Apple {}
    }

    // #[frb(rust_opaque_codec_moi)]
    pub fn apple_method_ref(&self) {}
}
