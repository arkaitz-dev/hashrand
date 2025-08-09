pub fn create_audit_logger(enabled: bool) -> impl Fn(&str) {
    move |msg: &str| {
        if enabled {
            use std::time::SystemTime;
            let now = SystemTime::now();
            if let Ok(duration) = now.duration_since(SystemTime::UNIX_EPOCH) {
                eprintln!("[AUDIT] {}: {}", duration.as_secs(), msg);
            } else {
                eprintln!("[AUDIT] {}", msg);
            }
        }
    }
}