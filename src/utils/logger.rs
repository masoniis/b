use time::macros::format_description;
use tracing_subscriber::{
    filter::EnvFilter, fmt, fmt::time::LocalTime, layer::SubscriberExt, prelude::*, Registry,
};

pub fn attach_logger() {
    let timer = LocalTime::new(format_description!("[hour repr:24]:[minute]:[second]"));
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_line_number(false)
        .with_thread_names(false)
        .with_file(false)
        .with_timer(timer)
        .compact();

    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    #[cfg(feature = "tracy")]
    let subscriber = subscriber.with(tracing_tracy::TracyLayer::default());

    subscriber.init();
}
