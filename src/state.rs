#[derive(Clone, Copy, Debug)]
pub enum Platform {
    Mac,
    Windows,
    Linux,
    Unknown,
}

impl Platform {
    pub fn detect() -> Self {
        match std::env::consts::OS {
            "macos" => Platform::Mac,
            "windows" => Platform::Windows,
            "linux" => Platform::Linux,
            _ => Platform::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Mac => "macOS",
            Platform::Windows => "Windows",
            Platform::Linux => "Linux",
            Platform::Unknown => "Unknown",
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub platform: Platform,
    pub watched_ports: Vec<u16>,
    pub started_at: std::time::Instant,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            platform: Platform::detect(),
            watched_ports: vec![3000, 5173, 8000, 8080, 11434],
            started_at: std::time::Instant::now(),
        }
    }
}
