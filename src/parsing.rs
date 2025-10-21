pub fn parse_size_value(size_str: &str) -> f64 {
    let size_str = size_str.trim().split_whitespace().next().unwrap_or("0");
    let (num_str, unit) = size_str.split_at(size_str.len() - 1);
    let num = num_str.parse().unwrap_or(0.0);

    match unit {
        "b" | "B" => num / (1024.0 * 1024.0),
        "k" | "K" => num / 1024.0,
        "m" | "M" => num,
        "g" | "G" => num * 1024.0,
        _ => num
    }
}

pub fn parse_percent_value(percent_str: &str) -> f64 {
    percent_str
        .trim()
        .trim_end_matches('%')
        .parse()
        .unwrap_or(0.0)
}