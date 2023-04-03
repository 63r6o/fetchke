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

impl Default for DeviceData {
    fn default() -> Self {
        Self::new()
    }
}

pub mod printer {
    use ansi_term::Style;

    pub fn print_info(
        info_to_print: &[(String, String)],
        decor: &[String],
        title_style: &Style,
    ) {
        let mut info_iter = info_to_print.iter().peekable();
        let mut decor_iter = decor.iter().peekable();
        let empty_info = ("".to_string(), "".to_string());
        let empty_decor = " ".repeat(decor[0].chars().count());

        while info_iter.peek().is_some() || decor_iter.peek().is_some() {
            let (title, info) = info_iter.next().unwrap_or(&empty_info);
            let decor_line = decor_iter.next().unwrap_or(&empty_decor);

            if title == "Name" || title == "Divider" {
                println!("{}{}", decor_line, info)
            } else {
                println!("{}{}: {}", decor_line, title_style.paint(title), info);
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
