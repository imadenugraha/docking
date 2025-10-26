# Docking

Docking is a simple Rust CLI tool to monitor resource usage (CPU, memory, network, and block I/O) of running containers using either Docker or Podman. It provides a summary of container statistics in a table, JSON, or detailed format.

## Features

- Detects and uses either Docker or Podman automatically
- Displays per-container resource usage:
	- CPU percentage
	- Memory usage and percentage
	- Network I/O (input/output)
	- Block I/O (read/write)
- Output formats:
	- Table (default)
	- JSON
	- Detailed per-container view

## Usage

### 1. Using Cargo

Build the project with Cargo:

```sh
cargo build --release
```

Run the tool (default output is a table):

```sh
./target/release/docking
```

You can specify the output format as the first argument:

```sh
./target/release/docking table      # Table view (default)
./target/release/docking json       # JSON output
./target/release/docking detailed   # Detailed per-container output
```

### 2. Using Make

Build based on your environment:

release:
```sh
make install-release
```

debug:
```sh
make install-debug
```

Run the tool (default output is a table):

release:
```sh
./usr/bin/docking
```

debug:
```sh
./docking
```

## Example Output

**Table format:**

```
╔════════════════╦═══════════╦════════════╦══════════╦═══════════╗
║ Container      ║ CPU (%)   ║ Memory (%) ║ Net I/O  ║ Block I/O ║
╠════════════════╬═══════════╬════════════╬══════════╬═══════════╣
║ my_container   ║ 2.50      ║ 10.00      ║ ↓1.2M ↑0.8M ║ R0.1M W0.0M ║
╚════════════════╩═══════════╩════════════╩══════════╩═══════════╝
```

**JSON format:**

```json
[
	{
		"container_id": "abcdef123456",
		"container_name": "my_container",
		"cpu_percent": 2.5,
		"memory_usage_mb": 128.0,
		"memory_limit_mb": 1024.0,
		"memory_percent": 10.0,
		"net_input_mb": 1.2,
		"net_output_mb": 0.8,
		"io_read_mb": 0.1,
		"io_write_mb": 0.0
	}
]
```

**Detailed format:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Container: my_container (abcdef123456)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
CPU Usage:        2.50%
Memory Usage:     128.00 MB / 1024.00 MB (10.00%)
Network I/O:      ↓ 1.20 MB (input) | ↑ 0.80 MB (output)
Block I/O:        ↓ 0.10 MB (read) | ↑ 0.00 MB (write)
```

## Requirements

- Docker or Podman installed and accessible in your PATH
- Rust toolchain (for building from source)

## License

Apache2 License