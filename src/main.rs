use ansi_term::{Colour, Style};
use sysinfo::{System, SystemExt, CpuExt};
use whoami::Platform;

mod utils;

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
        }
    }
}

fn print_rendered(rendered_info: &[(&str, &String)], decor: &String, title_style: &Style) {
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
fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    let info = Info::new(&system);

    let rendered_info = [
        ("Name", &info.name),
        ("Divider", &"-".repeat(info.name.len())),
        ("OS", &info.os),
        ("Kernel", &format!("{} {}", info.kernel_name, info.kernel)),
        ("DE", &info.de),
        ("Uptime", &utils::sec_to_string(info.uptime)),
        ("CPU", &info.cpu),
        ("Memory", &format!("{:.2}MiB / {:.0}MiB",
                utils::kb_to_mb(info.used_memory),
                utils::kb_to_mb(info.max_memory)
            ),
        ),
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
    print_rendered(&rendered_info, &decor, &title_style);
    println!();
}