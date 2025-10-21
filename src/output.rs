use serde_json::json;

use crate::stats::ContainerStats;

pub fn print_stats_table(stats: &[ContainerStats]) {
    println!("\n╔════════════════╦═══════════╦════════════╦══════════╦═══════════╗");
    println!("║ Container      ║ CPU (%)   ║ Memory (%) ║ Net I/O  ║ Block I/O ║");
    println!("╠════════════════╬═══════════╬════════════╬══════════╬═══════════╣");

    for stat in stats {
        let net_io = format!(
            "↓{:.1}M ↑{:.1}M",
            stat.net_input_mb, stat.net_output_mb
        );

        let block_io = format!(
            "R{:.1}M W{:.1}M",
            stat.io_read_mb, stat.io_write_mb
        );

        println!(
            "║ {:<14} ║ {:<9.2} ║ {:<10.2} ║ {:<8} ║ {:<9} ║",
            &stat.container_name[..std::cmp::min(14, stat.container_name.len())],
            stat.cpu_percent,
            stat.memory_percent,
            net_io,
            block_io
        );
    }

    println!("╚════════════════╩═══════════╩════════════╩══════════╩═══════════╝");
}

pub fn print_stats_json(stats: &[ContainerStats]) {
    let json_output = json!(stats);
    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
}

pub fn print_stats_detailed(stats: &[ContainerStats]) {
    for stat in stats {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Container: {} ({})", stat.container_name, stat.container_id);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("CPU Usage:        {:.2}%", stat.cpu_percent);
        println!(
            "Memory Usage:     {:.2} MB / {:.2} MB ({:.2}%)",
            stat.memory_usage_mb, stat.memory_limit_mb, stat.memory_percent
        );
        println!(
            "Network I/O:      ↓ {:.2} MB (input) | ↑ {:.2} MB (output)",
            stat.net_input_mb, stat.net_output_mb
        );
        println!(
            "Block I/O:        ↓ {:.2} MB (read) | ↑ {:.2} MB (write)",
            stat.io_read_mb, stat.io_write_mb
        );
    }
}
