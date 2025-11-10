use std::fs;
use tracing::{debug, warn};

#[derive(Default, Debug)]
pub struct CpuInfo {
    pub name: String,
    pub usage_percent: f32,

    prev_idle: u64,
    prev_total: u64,
    initialized: bool,
}

impl CpuInfo {
    pub fn new() -> Self {
        let name = Self::read_cpu_model_name();
        let mut info = Self {
            name,
            ..Default::default()
        };
        if let Some((idle, total)) = Self::read_cpu_times() {
            info.prev_idle = idle;
            info.prev_total = total;
            info.initialized = true;
        }
        info
    }

    pub fn parse(&mut self) {
        if let Some((current_idle, current_total)) = Self::read_cpu_times() {
            if self.initialized && self.prev_total > 0 {
                let idle_diff = current_idle.saturating_sub(self.prev_idle);
                let total_diff = current_total.saturating_sub(self.prev_total);

                if total_diff > 0 {
                    let idle_ratio = idle_diff as f64 / total_diff as f64;
                    let usage = (1.0 - idle_ratio) * 100.0;
                    self.usage_percent = usage as f32;
                    self.usage_percent = self.usage_percent.clamp(0.0, 100.0);
                }
            }

            self.prev_idle = current_idle;
            self.prev_total = current_total;
            self.initialized = true;
        } else {
            warn!("Failed to read CPU times from /proc/stat");
        }
    }

    fn read_cpu_model_name() -> String {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(content) => {
                debug!("Successfully read /proc/cpuinfo");
                for line in content.lines() {
                    if line.starts_with("model name") {
                        if let Some((_, value)) = line.split_once(':') {
                            return value.trim().to_string();
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Can't read /proc/cpuinfo: {}", e);
            }
        }
        "Unknown CPU".to_string()
    }

    fn read_cpu_times() -> Option<(u64, u64)> {
        let content = fs::read_to_string("/proc/stat").ok()?;
        let line = content.lines().find(|l| l.starts_with("cpu "))?;

        let mut v: Vec<u64> = line
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        if v.len() < 4 {
            return None;
        }

        let user = v[0];
        let nice = v[1];
        let system = v[2];
        let idle = v[3];
        let iowait = *v.get(4).unwrap_or(&0);
        let irq = *v.get(5).unwrap_or(&0);
        let softirq = *v.get(6).unwrap_or(&0);
        let steal = *v.get(7).unwrap_or(&0);

        let idle_time = idle.saturating_add(iowait);
        let total_time = user
            .saturating_add(nice)
            .saturating_add(system)
            .saturating_add(idle_time)
            .saturating_add(irq)
            .saturating_add(softirq)
            .saturating_add(steal);

        Some((idle_time, total_time))
    }
}
