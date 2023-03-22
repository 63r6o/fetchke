use ansi_term::{Colour, Style};
use fetchke::{printer, utils, DeviceData};
use whoami::Platform;

fn main() {
    let device = DeviceData::new();
    let divider_length = device.name.len();

    let info_to_render = vec![
        ("Name", device.name),
        ("Divider", "-".repeat(divider_length)),
        ("OS", device.os),
        (
            "Kernel",
            format!("{} {}", device.kernel_name, device.kernel),
        ),
        ("DE", device.de),
        ("Uptime", utils::sec_to_string(device.uptime)),
        ("CPU", device.cpu),
        (
            "Memory",
            format!(
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

    let decor = os_emoji.repeat(40);
    let title_style = Style::new().bold().fg(Colour::Purple);

    println!();
    printer::print_rendered(&info_to_render, &decor, &title_style);
    println!();
}
