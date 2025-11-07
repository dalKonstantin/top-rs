use std::fs;

#[derive(Default, Debug)]
pub struct MemInfo {
    pub total_kb: u64,
    pub free_kb: u64,
    pub available_kb: u64,
}

impl MemInfo {
    pub fn new() -> Self {
        let mut m = MemInfo::default();
        m.parse();
        m
    }
    fn parse(&mut self) {
        let content = match fs::read_to_string("out.txt") {
            Ok(data) => data,
            Err(_) => String::new(),
        };

        for line in content.lines() {
            let mut parts = line.split_whitespace();
            let key = parts.next();
            let value_str = parts.next();

            if let (Some(field), Some(value)) = (key, value_str) {
                let val_kb = value.parse().unwrap_or(0);
                match field {
                    "MemTotal:" => self.total_kb = val_kb,
                    "MemFree:" => self.free_kb = val_kb,
                    "MemAvailable:" => self.available_kb = val_kb,
                    _ => {}
                }
            }
        }
    }
}
