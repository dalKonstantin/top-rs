use std::fs;
use tracing::{debug, field::debug, warn};

#[derive(Default)]
pub struct CpuInfo {
    pub name: String,
}

impl CpuInfo {
    pub fn new() -> Self {
        let mut c = CpuInfo::default();
        c.name = Self::read_cpu_model_name();
        c
    }

    pub fn parse(&mut self) {}

    fn read_cpu_model_name() -> String {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(content) => {
                debug!("Sucessfully read /proc/cpuinfo");
                for line in content.lines() {
                    if line.starts_with("model name") {
                        if let Some((_, value)) = line.split_once(":") {
                            return value.trim().to_string();
                        }
                    }
                }
            }
            Err(_) => return "Unknown CPU".to_string(),
        }

        "Unknown CPU".to_string()
    }
}
