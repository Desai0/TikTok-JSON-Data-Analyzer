// chart_utils.rs

use std::collections::HashMap;
use textplots::{Chart, Plot, Shape};

pub fn print_dms_chart(dms: &HashMap<String, usize>) {
    if dms.is_empty() {
        return;
    }
    println!("\n--- DM Distribution ---");
    let mut sorted_dms: Vec<(&String, &usize)> = dms.iter().collect();
    sorted_dms.sort_by(|a, b| b.1.cmp(a.1));

    let top_dms: Vec<_> = sorted_dms.iter().take(10).collect();
    let max_len = top_dms.iter().map(|(name, _)| name.len()).max().unwrap_or(0);

    // Find the max count to scale the bars relative to the top user
    let max_count = top_dms.first().map(|(_, count)| **count).unwrap_or(1) as f64;
    let max_bar_width = 40.0; // Max width for a bar in characters

    for (chat_name, count) in top_dms {
        let bar_len = ((**count as f64 / max_count) * max_bar_width) as usize;
        let bar = "█".repeat(bar_len);
        println!("{:<width$}: |{} {}", chat_name, bar, count, width = max_len);
    }
    println!("-----------------------\n");
}

pub fn print_time_spent_chart(time_in_minutes: usize) {
    println!("\n--- Daily Time Spent (minutes) ---");
    Chart::new(120, 60, 0.0, 100.0)
        .lineplot(&Shape::Bars(&[
            (0.0, 0.0),
            (20.0, time_in_minutes as f32),
            (40.0, 0.0)
        ]))
        .lineplot(&Shape::Bars(&[
            (50.0, 0.0),
            (70.0, 80.0), // Average time spent in World
            (90.0, 0.0)
        ]))
        .display();
    println!("   Your Time ({})   |   Average in World (~80)", time_in_minutes);
    println!("------------------------------------\n");
} 