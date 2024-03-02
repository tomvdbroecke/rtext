// Uses
use std::time::Duration;

// Format time function
pub(crate) fn format_time(&duration: &Duration) -> String {
    // Get the seconds
    let seconds = duration.as_secs();

    // Display milliseconds if seconds is 0
    if seconds == 0 {
        let milliseconds = duration.subsec_millis();
        return format!("{}ms", milliseconds)
    }

    // Calculate the amount of days, hours, minutes and remaining seconds
    let days = seconds / (24 * 3600);
    let hours = (seconds % (24 * 3600)) / 3600;
    let minutes = (seconds % 3600) / 60;
    let remaining_seconds = seconds % 60;

    // Match the appropriate format
    match (days, hours, minutes, remaining_seconds) {
        (0, 0, 0, s) => format!("{}s", s),
        (0, 0, m, s) => format!("{}m {}s", m, s),
        (0, h, m, s) => format!("{}h {}m {}s", h, m, s),
        (d, h, m, s) => format!("{}d {}h {}m {}s", d, h, m, s),
    }
}