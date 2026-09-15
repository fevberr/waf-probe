pub struct Probe0 {
    pub kind: &'static str,
    pub path: &'static str,
}

pub const PROBES0: &[Probe0] = &[
    Probe0 { kind: "xss-marker", path: "/%3Cwafprobe%3Ealert%3C/wafprobe%3E" },
    Probe0 { kind: "sqli-marker", path: "/wafprobe?id=1%27%20OR%20%271%27%3D%271" },
    Probe0 { kind: "traversal-marker", path: "/wafprobe?id=..%2F..%2F..%2F..%2Fetc%2Fhostname" },
    Probe0 { kind: "sqli-union-marker", path: "/wafprobe?id=1%20UNION%20SELECT%20NULL--" },
];