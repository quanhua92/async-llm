pub fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

    tracing_log::LogTracer::init().expect("Failed to set up LogTracer");

    let subscriber = Registry::default()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(
            fmt::layer()
                .compact()
                .with_file(true)
                .with_line_number(true)
                .with_target(false),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up subscriber")
}
