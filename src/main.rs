use ansi_term::{Colour, Style};
use sysinfo::{CpuExt, System, SystemExt};
use whoami::Platform;

struct Info {
    name: String,
    platform: Platform,
    os: String,
    kernel_name: String,
    kernel: String,
    de: String,
    uptime: u64,
    cpu: String,
    max_memory: u64,
    used_memory: u64,
    // max_swap: u64,
    // used_swap: u64
}

impl Info {
    fn new(system: &System) -> Self {
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
            // max_swap : system.total_swap(),
            // used_swap : system.used_swap()
        }
    }
}

fn main() {
    // todo: temps
    // todo: functionality to switch between gb and mb ram

    let mut system = System::new_all();
    system.refresh_all();

    let info = Info::new(&system);

    let rendered_info = [
        ("Name", &info.name),
        ("Divider", &"-".repeat(info.name.len())),
        ("OS", &info.os),
        ("Kernel", &format!("{} {}", info.kernel_name, info.kernel)),
        ("DE", &info.de),
        ("Uptime", &sec_to_string(info.uptime)),
        ("CPU", &info.cpu),
        ("Memory", &format!("{:.2}MiB / {:.0}MiB",
                kb_to_mb(info.used_memory),
                kb_to_mb(info.max_memory)
            ),
        ),
        // ("Swap", &format!("{:.2}GB / {:.0}GB", kb_to_gb(info.used_swap), kb_to_gb(info.max_swap)))
    ];

    let os_emoji = match info.platform {
        Platform::MacOS => "",
        Platform::Linux => "🐧",
        Platform::Windows => "🪟",
        _ => "✖",
    };

    let title_style = Style::new().bold().fg(Colour::Purple);
    let decor = format!("{}", os_emoji.repeat(40));

    println!();
    for (i, item) in rendered_info.iter().enumerate() {
        match i {
            0 | 1 => println!("{}  {}", decor, item.1),
            _ => {
                let line = format!("{}  {}: {}", decor, title_style.paint(item.0), item.1);
                println!("{}", line);
            }
        }
    }
    println!();
}

// todo
// refactor utils to a different file
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
