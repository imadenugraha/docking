use std::process::Command;
use crate::stats::ContainerStats;
use crate::parsing::{parse_size_value, parse_percent_value};

pub fn get_container_runtime() -> String {
    let docker_check = Command::new("docker")
        .arg("--version")
        .output();

    match docker_check {
        Ok(_) => "docker".to_string(),
        Err(_) => {
            let podman_check = Command::new("podman")
                .arg("--version")
                .output();

            match podman_check {
                Ok(_) => "podman".to_string(),
                Err(_) => {
                    eprintln!("Error: Docker or podman not exist!");
                    std::process::exit(1);
                }
            }
        }
    }
}

pub fn fetch_container_stats(runtime: &str) -> Result<Vec<ContainerStats>, Box<dyn std::error::Error>> {
    let output = Command::new(runtime)
        .arg("--no-stream")
        .arg("--format=json")
        .arg("--all")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Failed executing {}: {}", runtime, stderr);
        return Err(format!("Failed to get stats from {}", runtime).into());
    };

    let stdout = String::from_utf8(output.stdout)?;

    let stats: Vec<serde_json::Value> = serde_json::from_str(&stdout)?;

    let mut container_stats = Vec::new();

    for stat in stats {
        let container_id = stat["ID"]
            .as_str()
            .unwrap_or("unknown")
            .chars()
            .take(12)
            .collect();

        let container_name = stat["Names"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let cpu_str = stat["CPUPerc"].as_str().unwrap_or("0%");
        let mem_usage = stat["MemUsage"].as_str().unwrap_or("0M");
        let mem_percent = stat["MemPerc"].as_str().unwrap_or("0%");
        let net_input = stat["NetInput"].as_str().unwrap_or("0B");
        let net_output = stat["NetOutput"].as_str().unwrap_or("0B");
        let io_read = stat["BlockInput"].as_str().unwrap_or("0B");
        let io_write = stat["BlockOutput"].as_str().unwrap_or("0B");

        let cpu_percent = parse_percent_value(cpu_str);
        let memory_percent = parse_percent_value(mem_percent);

        let mem_parts: Vec<&str> = mem_usage.split("/").collect();
        let memory_usage_mb = if mem_parts.len() > 0 {
            parse_size_value(mem_parts[0])
        } else {
            0.0
        };

        let memory_limit_mb = if mem_parts.len() > 1 {
            parse_size_value(mem_parts[1])
        } else {
            0.0
        };

        let net_input_mb = parse_size_value(net_input);
        let net_output_mb = parse_size_value(net_output);
        let io_read_mb = parse_size_value(io_read);
        let io_write_mb = parse_size_value(io_write);

        container_stats.push(ContainerStats {
            container_id,
            container_name,
            cpu_percent,
            memory_percent,
            memory_usage_mb,
            memory_limit_mb,
            net_input_mb,
            net_output_mb,
            io_read_mb,
            io_write_mb
        });
    }

    Ok(container_stats)
}
