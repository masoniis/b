use time::macros::format_description;
use tracing_subscriber;
use tracing_subscriber::fmt::time::LocalTime;

pub fn attach_logger() {
    let timer = LocalTime::new(format_description!("[hour repr:24]:[minute]:[second]"));

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_line_number(false)
        .with_thread_names(false)
        .with_file(true)
        .with_timer(timer)
        .compact()
        .init();
}
