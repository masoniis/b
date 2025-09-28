// in src/utils/logger.rs

// This attribute tells the compiler to only include this module
// when the target architecture is NOT wasm32.
#[cfg(not(target_arch = "wasm32"))]
pub mod native_logger {
    use time::macros::format_description;
    use tracing_subscriber::{filter::EnvFilter, fmt::time::LocalTime};

    pub fn attach_logger() {
        let timer = LocalTime::new(format_description!("[hour repr:24]:[minute]:[second]"));
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(true)
            .with_line_number(false)
            .with_thread_names(false)
            .with_file(false)
            .with_timer(timer)
            .compact()
            .init();
    }
}
