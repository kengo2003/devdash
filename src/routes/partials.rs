use askama::Template;
use axum::{extract::State, response::Html};
use std::result::Result::Ok;
use std::sync::Arc;

use crate::state::AppState;

use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

#[derive(Template)]
#[template(path = "partials/metrics.html")]
struct MetricsTemplate {
    cpu_percent: String,
    mem_text: String,
    swap_text: String,
    uptime_secs: u64,
    swap_used_nonzero: bool,
}

pub async fn metrics(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let cpu = sys.global_cpu_usage();

    let mem_total = sys.total_memory() as f64; // KiB
    let mem_used = sys.used_memory() as f64;

    let swap_total = sys.total_swap() as f64;
    let swap_used = sys.used_swap() as f64;

    let to_gb = |kib: f64| kib / 1024.0 / 1024.0;

    let mem_percent = if mem_total > 0.0 {
        mem_used / mem_total * 100.0
    } else {
        0.0
    };

    let cpu_percent = format!("{:.1}%", cpu);
    let mem_text = format!(
        "{:.2} / {:.2} GB ({:.1}%)",
        to_gb(mem_used),
        to_gb(mem_total),
        mem_percent
    );
    let swap_text = format!("{:.2} / {:.2} GB", to_gb(swap_used), to_gb(swap_total));

    let uptime_secs = state.started_at.elapsed().as_secs();

    let t = MetricsTemplate {
        cpu_percent,
        mem_text,
        swap_text,
        uptime_secs,
        swap_used_nonzero: swap_used > 0.0,
    };

    Html(t.render().unwrap())
}

#[derive(Template)]
#[template(path = "partials/ports_watch.html")]
struct PortsWatchTemplate {
    pub ports: Vec<PortRow>,
}

#[derive(Clone, Debug)]
pub struct PortRow {
    port: u16,
    open: bool,
}

async fn is_port_open(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    matches!(
        timeout(Duration::from_millis(150), TcpStream::connect(addr)).await,
        Ok(Ok(_))
    )
}

pub async fn ports_watch(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut rows = Vec::with_capacity(state.watched_ports.len());
    for &p in &state.watched_ports {
        let open = is_port_open(p).await;
        rows.push(PortRow { port: p, open });
    }

    let t = PortsWatchTemplate { ports: rows };
    Html(t.render().unwrap())
}
