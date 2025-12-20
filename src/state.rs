use std::sync::Mutex;

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

#[derive(Debug)]
pub struct AppState {
    pub platform: Platform,
    pub watched_ports: Vec<u16>,
    pub started_at: std::time::Instant,
    pub sys: Mutex<sysinfo::System>,
}

impl AppState {
    pub fn new() -> Self {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_cpu_all();
        sys.refresh_memory();
        Self {
            platform: Platform::detect(),
            watched_ports: vec![3000, 5173, 8000, 8080, 11434],
            started_at: std::time::Instant::now(),
            sys: Mutex::new(sys),
        }
    }
}
