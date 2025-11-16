use std::fs;
use tracing::warn;

use crate::proc;

pub struct ProcessInfo {
    pub pid: u32,
    pub command: String,
    //cpu: f32,
    //pub mem: u64,
}

impl ProcessInfo {
    pub fn from_pid(pid: u32) -> Option<Self> {
        let command = get_process_name(pid);
        if command == "[unknown]" {
            return None;
        }

        Some(ProcessInfo { pid, command })
    }
}

pub fn list_pids() -> Vec<u32> {
    let mut pids = Vec::new();
    if let Ok(entries) = fs::read_dir("/proc/") {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Ok(pid) = file_name.to_string_lossy().parse::<u32>() {
                    pids.push(pid);
                }
            }
        }
    }

    pids.sort_unstable();
    pids
}

fn get_process_name(pid: u32) -> String {
    let path = format!("/proc/{}/comm", pid);
    fs::read_to_string(path).map_or("[unknown]".to_string(), |s| s.trim().to_string())
}

pub fn collect_processes() -> Vec<ProcessInfo> {
    let mut processes = Vec::new();
    for pid in list_pids() {
        if let Some(proc_info) = ProcessInfo::from_pid(pid) {
            processes.push(proc_info);
        }
    }

    processes
}

#[cfg(test)]
#[test]
fn test_list_pids() {
    let pids = list_pids();
    println!("{:?}", pids);
    assert_ne!(pids.len(), 0);
}
