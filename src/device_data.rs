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
