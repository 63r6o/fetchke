use ansi_term::{Colour, Style};
use whoami::Platform;

mod device_data;
mod utils;

fn main() {
    let device = device_data::DeviceData::new();

    let rendered_info = [
        ("Name", &device.name),
        ("Divider", &"-".repeat(device.name.len())),
        ("OS", &device.os),
        (
            "Kernel",
            &format!("{} {}", device.kernel_name, device.kernel),
        ),
        ("DE", &device.de),
        ("Uptime", &utils::sec_to_string(device.uptime)),
        ("CPU", &device.cpu),
        (
            "Memory",
            &format!(
                "{:.2}MiB / {:.0}MiB",
                utils::kb_to_mb(device.used_memory),
                utils::kb_to_mb(device.max_memory)
            ),
        ),
    ];

    let os_emoji = match device.platform {
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
