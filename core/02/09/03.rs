pub fn blocked0(status: u16, body: &str, baseline: u16) -> (bool, String) {
    match status {
        403 | 406 | 419 | 429 | 501 | 503 => return (true, format!("status {}", status)),
        _ => {}
    }
    let low = body.to_lowercase();
    let sigs = [
        "access denied",
        "request blocked",
        "not acceptable",
        "attention required",
        "modsecurity",
        "incapsula incident",
        "the requested url was rejected",
        "just a moment",
        "cf-chl-",
    ];
    for s in sigs {
        if low.contains(s) {
            return (true, format!("body contains {:?}", s));
        }
    }
    if baseline != 0 && status != baseline {
        return (true, format!("status diverged ({} -> {})", baseline, status));
    }
    (false, "passed".to_string())
}