use ansi_term::{Colour, Style};
use fetchke::{DeviceData, utils, printer};
use whoami::Platform;

fn main() {
    let device = DeviceData::new();

    let info_to_render = [
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
    printer::print_rendered(&info_to_render, &decor, &title_style);
    println!();
}
