use ansi_term::{Colour, Style};
use fetchke::{printer, utils, DeviceData};
use whoami::Platform;

fn main() {
    let device = DeviceData::new();
    let divider_length = device.name.len();

    let info_to_print = vec![
        ("Name".to_string(), device.name),
        ("Divider".to_string(), "-".repeat(divider_length)),
        ("OS".to_string(), device.os),
        (
            "Kernel".to_string(),
            format!("{} {}", device.kernel_name, device.kernel),
        ),
        ("DE".to_string(), device.de),
        ("Uptime".to_string(), utils::sec_to_string(device.uptime)),
        ("CPU".to_string(), device.cpu),
        (
            "Memory".to_string(),
            format!(
                "{:.2}MiB / {:.0}MiB",
                utils::kb_to_mb(device.used_memory),
                utils::kb_to_mb(device.max_memory)
            ),
        ),
    ];

    let os_decor = match device.platform {
        Platform::MacOS => [
            "███╗   ███╗ █████╗  ██████╗   ",
            "████╗ ████║██╔══██╗██╔════╝   ",
            "██╔████╔██║███████║██║        ",
            "██║╚██╔╝██║██╔══██║██║        ",
            "██║ ╚═╝ ██║██║  ██║╚██████╗   ",
            "╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝   ",
        ],
        Platform::Linux => [
            "██╗     ██╗███╗   ██╗   ",
            "██║     ██║████╗  ██║   ",
            "██║     ██║██╔██╗ ██║   ",
            "██║     ██║██║╚██╗██║   ",
            "███████╗██║██║ ╚████║   ",
            "╚══════╝╚═╝╚═╝  ╚═══╝   ",
        ],
        Platform::Bsd => [
            "██████╗ ███████╗██████╗    ",
            "██╔══██╗██╔════╝██╔══██╗   ",
            "██████╔╝███████╗██║  ██║   ",
            "██╔══██╗╚════██║██║  ██║   ",
            "██████╔╝███████║██████╔╝   ",
            "╚═════╝ ╚══════╝╚═════╝    ",
        ],
        Platform::Windows => [
            "██╗    ██╗██╗███╗   ██╗   ",
            "██║    ██║██║████╗  ██║   ",
            "██║ █╗ ██║██║██╔██╗ ██║   ",
            "██║███╗██║██║██║╚██╗██║   ",
            "╚███╔███╔╝██║██║ ╚████║   ",
            " ╚══╝╚══╝ ╚═╝╚═╝  ╚═══╝   ",
        ],
        _ => [
            "██████╗ ██████╗ ██████╗    ",
            "╚════██╗╚════██╗╚════██╗   ",
            "  ▄███╔╝  ▄███╔╝  ▄███╔╝   ",
            "  ▀▀══╝   ▀▀══╝   ▀▀══╝    ",
            "  ██╗     ██╗     ██╗      ",
            "  ╚═╝     ╚═╝     ╚═╝      ",
        ],
    };
    let string_os_decor = os_decor.map(|line| line.to_string());

    let title_style = Style::new().bold().fg(Colour::Purple);

    println!();
    printer::print_info(&info_to_print, &string_os_decor, &title_style);
    println!();
}
