use sysinfo::{CpuExt, System, SystemExt};
use whoami::Platform;

pub struct DeviceData {
    pub name: String,
    pub platform: Platform,
    pub os: String,
    pub kernel_name: String,
    pub kernel: String,
    pub de: String,
    pub uptime: u64,
    pub cpu: String,
    pub max_memory: u64,
    pub used_memory: u64,
}

impl DeviceData {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            name: whoami::username(),
            platform: whoami::platform(),
            os: system
                .long_os_version()
                .unwrap_or_else(|| String::from("unknown")),
            kernel: system
                .kernel_version()
                .unwrap_or_else(|| String::from("unknown")),
            kernel_name: system.name().unwrap_or_else(|| String::from("unknown")),
            de: whoami::desktop_env().to_string(),
            uptime: system.uptime(),
            cpu: system.global_cpu_info().brand().to_string(),
            max_memory: system.total_memory(),
            used_memory: system.used_memory(),
        }
    }
}

pub mod printer {
    use ansi_term::Style;

    pub fn print_rendered(rendered_info: &[(&str, &String)], decor: &String, title_style: &Style) {
        for (i, item) in rendered_info.iter().enumerate() {
            match i {
                0 | 1 => println!("{}  {}", decor, item.1),
                _ => {
                    let line = format!("{}  {}: {}", decor, title_style.paint(item.0), item.1);
                    println!("{}", line);
                }
            }
        }
    }
}

pub mod utils {
    pub fn kb_to_gb(kb: u64) -> f64 {
        (kb as f64) / 1048576.0
    }

    pub fn kb_to_mb(kb: u64) -> f64 {
        (kb as f64) / 1024.0
    }

    pub fn sec_to_string(sec: u64) -> String {
        let days = sec / (24 * 60 * 60);
        let hours = (sec % (24 * 60 * 60)) / (60 * 60);
        let minutes = (sec % (60 * 60)) / 60;

        if hours < 1 {
            format!("{minutes} minutes")
        } else if days < 1 {
            format!("{hours} hours, {minutes} minutes")
        } else {
            format!("{days} days, {hours} hours, {minutes} minutes")
        }
    }
}
