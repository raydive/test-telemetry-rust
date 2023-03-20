use opentelemetry::{global, sdk::export::trace::stdout, trace::{Tracer, TraceContextExt}, KeyValue, Key, Value};

fn main() {
    // Create a new trace pipeline that prints to stdout
    let tracer = stdout::new_pipeline().install_simple();

    tracer.in_span("doing_work", |cx| {
        // Traced app logic here...
        let span = cx.span();
        span.set_attribute(KeyValue { key: Key::new("mister"), value: Value::I64(1)});
        span.add_event("test telemetry", vec![KeyValue::new("trace1", 1)]);
    });

    // Shutdown trace pipeline
    global::shutdown_tracer_provider();
}