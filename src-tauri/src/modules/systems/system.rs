use serde::Serialize;
use std::process::Command;

/// Basic system information and hardware specifications.
#[derive(Serialize)]
pub struct SystemInfo {
    hostname: String,
    os_name: String,
    kernel_version: String,
    cpu_model: String,
    memory_total: String,
    gpu_info: String,
    disk_total: String,
    disk_used: String,
    disk_percent: u8,
}

/// Gathers comprehensive system information by querying various system tools and files.
#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    let hostname = run_cmd("hostname");

    let mut os_name = "Unknown".to_string();
    let mut kernel_version = "Unknown".to_string();

    // Query hostnamectl for OS and Kernel information
    if let Ok(output) = Command::new("hostnamectl").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("Operating System:") {
                os_name = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("Kernel:") {
                kernel_version = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
    }

    // Fallback to /etc/os-release if hostnamectl failed to provide OS name
    if os_name == "Unknown" || os_name.is_empty() {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    os_name = line.replace("PRETTY_NAME=", "").replace("\"", "");
                    break;
                }
            }
        }
    }

    // Fallback to uname for kernel version
    if kernel_version == "Unknown" || kernel_version.is_empty() {
        kernel_version = run_cmd("uname -r");
    }

    // Retrieve CPU model name
    let mut cpu_model = "Unknown".to_string();
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("grep -m 1 'model name' /proc/cpuinfo | cut -d: -f2")
        .output()
    {
        cpu_model = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }

    // Retrieve total system memory
    let mut memory_total = "Unknown".to_string();
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("free -h | awk '/^Mem:/ {print $2}'")
        .output()
    {
        memory_total = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }

    // Retrieve primary GPU information using lspci
    let mut gpu_info = "Unknown".to_string();
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|3d' | cut -d: -f3 | head -n 1")
        .output()
    {
        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !raw.is_empty() {
            gpu_info = raw;
        }
    }

    // Retrieve disk usage for /
    let mut disk_total = "Unknown".to_string();
    let mut disk_used = "Unknown".to_string();
    let mut disk_percent: u8 = 0;

    if let Ok(output) = Command::new("df").arg("-h").arg("/").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.len() >= 2 {
            let parts: Vec<&str> = lines[1].split_whitespace().collect();
            if parts.len() >= 5 {
                disk_total = parts[1].to_string();
                disk_used = parts[2].to_string();
                let percent_str = parts[4].trim_end_matches('%');
                disk_percent = percent_str.parse().unwrap_or(0);
            }
        }
    }

    SystemInfo {
        hostname,
        os_name,
        kernel_version,
        cpu_model,
        memory_total,
        gpu_info,
        disk_total,
        disk_used,
        disk_percent,
    }
}

/// Executes a shell command and returns the trimmed standard output.
fn run_cmd(cmd: &str) -> String {
    if let Some((prog, args)) = cmd.split_once(' ') {
        Command::new(prog)
            .arg(args)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default()
    } else {
        Command::new(cmd)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default()
    }
}

/// Retrieves the current GTK theme preference (dark or light) via GSettings.
#[tauri::command]
pub fn get_gtk_theme() -> String {
    let output = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "color-scheme"])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let raw = stdout.trim().replace("'", "");
            if raw == "prefer-dark" {
                "dark".to_string()
            } else {
                "light".to_string()
            }
        }
        Err(_) => "light".to_string(),
    }
}

/// Checks if a specific application (command) is installed/available in the system PATH.
#[tauri::command]
pub fn check_app_installed(app_name: String) -> bool {
    Command::new("which")
        .arg(&app_name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
