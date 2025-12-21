use askama::Template;
use axum::{extract::State, response::Html};
use std::result::Result::Ok;
use std::sync::Arc;
use sysinfo::ProcessesToUpdate;

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

#[derive(Clone, Debug)]
pub struct ProcRow {
    pub pid: i32,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_mb: f32,
    pub cpu_text: String,
    pub mem_text: String,
}

fn proc_row_from(sys_proc: &sysinfo::Process, pid: sysinfo::Pid) -> ProcRow {
    let pid_i32 = pid.as_u32() as i32;

    let mem_mb = (sys_proc.memory() as f64 / 1024.0) as f32;
    let cpu = sys_proc.cpu_usage();

    ProcRow {
        pid: pid_i32,
        name: sys_proc.name().to_string_lossy().to_string(),
        cpu_percent: cpu,
        mem_mb,
        cpu_text: format!("{:.1}%", cpu),
        mem_text: format!("{:.0} MB", mem_mb),
    }
}

fn top_procs_by_cpu(sys: &mut sysinfo::System, n: usize) -> Vec<ProcRow> {
    sys.refresh_processes(ProcessesToUpdate::All);
    sys.refresh_cpu_all();

    let mut rows: Vec<ProcRow> = Vec::new();
    for (pid, p) in sys.processes() {
        rows.push(proc_row_from(p, *pid));
    }

    rows.sort_by(|a, b| {
        b.cpu_percent
            .partial_cmp(&a.cpu_percent)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    rows.truncate(n);
    rows
}

fn top_procs_by_mem(sys: &mut sysinfo::System, n: usize) -> Vec<ProcRow> {
    sys.refresh_processes(ProcessesToUpdate::All);

    let mut rows: Vec<ProcRow> = Vec::new();
    for (pid, p) in sys.processes() {
        rows.push(proc_row_from(p, *pid));
    }

    rows.sort_by(|a, b| {
        b.mem_mb
            .partial_cmp(&a.mem_mb)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    rows.truncate(n);
    rows
}

#[derive(Template)]
#[template(path = "partials/top_procs.html")]
struct TopProcsTemplate {
    pub top_cpu: Vec<ProcRow>,
    pub top_mem: Vec<ProcRow>,
}

pub async fn top_procs(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut sys = state.sys.lock().unwrap();
    let top_cpu = top_procs_by_cpu(&mut sys, 10);
    let top_mem = top_procs_by_mem(&mut sys, 10);

    let t = TopProcsTemplate { top_cpu, top_mem };
    Html(t.render().unwrap())
}
