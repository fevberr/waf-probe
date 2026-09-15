pub fn now0() -> String {
    chrono::Utc::now().to_rfc3339()
}