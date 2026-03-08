use std::env;

pub fn init() {
    if env::var("RUST_LOG").is_err() {
        unsafe {
            #[cfg(debug_assertions)]
            env::set_var("RUST_LOG", "debug");
            #[cfg(not(debug_assertions))]
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
}
