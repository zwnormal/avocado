use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_subscriber() {
    LogTracer::init().expect("Failed to set logger");
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .finish();
    set_global_default(subscriber).expect("Failed to set subscriber");
}
