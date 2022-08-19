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
