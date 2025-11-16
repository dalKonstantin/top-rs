pub mod cpu_info;
pub mod mem_info;
pub mod process_info;

pub use cpu_info::CpuInfo;
pub use mem_info::MemInfo;
pub use process_info::{ProcessInfo, list_pids};

//use crate::proc::proc::CpuTimes;
