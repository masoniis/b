use time::macros::format_description;
use tracing_subscriber::{filter::EnvFilter, fmt::time::LocalTime};

pub fn attach_logger() {
    let timer = LocalTime::new(format_description!("[hour repr:24]:[minute]:[second]"));

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_line_number(false)
        .with_thread_names(false)
        .with_file(true)
        .with_timer(timer)
        .compact()
        .init();
}
