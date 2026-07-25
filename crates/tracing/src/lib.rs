use uuid::Uuid;

pub struct Tracer;

impl Tracer {
    pub fn new() -> Self {
        Tracer
    }

    pub fn start_span(&self, name: &str) -> String {
        let span_id = Uuid::new_v4().to_string();
        println!("[TRACE] Span started: {} (ID: {})", name, &span_id[..8]);
        span_id
    }

    pub fn end_span(&self, _span_id: &str) {
        // Placeholder
    }
}
