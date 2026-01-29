use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct SystemInfo {
    hostname: String,
    os_name: String,
    kernel_version: String,
    cpu_model: String,
    memory_total: String,
    gpu_info: String,
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    let hostname = run_cmd("hostname");

    // OS and Kernel from hostnamectl or files
    let mut os_name = "Unknown".to_string();
    let mut kernel_version = "Unknown".to_string();

    // Try hostnamectl first
    let output = Command::new("hostnamectl").output();
    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        for line in stdout.lines() {
            if line.contains("Operating System:") {
                os_name = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("Kernel:") {
                kernel_version = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
    }

    // Fallback if empty (e.g. if hostnamectl is not installed or different output format)
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
    if kernel_version == "Unknown" || kernel_version.is_empty() {
        kernel_version = run_cmd("uname -r");
    }

    // CPU
    let mut cpu_model = "Unknown".to_string();
    let cpu_output = Command::new("sh")
        .arg("-c")
        .arg("grep -m 1 'model name' /proc/cpuinfo | cut -d: -f2")
        .output();
    if let Ok(o) = cpu_output {
        cpu_model = String::from_utf8_lossy(&o.stdout).trim().to_string();
    }

    // Memory
    let mut memory_total = "Unknown".to_string();
    let mem_output = Command::new("sh")
        .arg("-c")
        .arg("free -h | awk '/^Mem:/ {print $2}'")
        .output();
    if let Ok(o) = mem_output {
        memory_total = String::from_utf8_lossy(&o.stdout).trim().to_string();
    }

    // GPU (lspci)
    let mut gpu_info = "Unknown".to_string();
    let gpu_output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|3d' | cut -d: -f3 | head -n 1")
        .output();
    if let Ok(o) = gpu_output {
        let raw = String::from_utf8_lossy(&o.stdout).trim().to_string();
        if !raw.is_empty() {
            gpu_info = raw;
        }
    }

    SystemInfo {
        hostname,
        os_name,
        kernel_version,
        cpu_model,
        memory_total,
        gpu_info,
    }
}

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

#[tauri::command]
pub fn get_gtk_theme() -> String {
    let output = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "color-scheme"])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            // Returns 'default' or 'prefer-dark' usually (with quotes)
            let raw = stdout.trim().replace("'", "");
            if raw == "prefer-dark" {
                "dark".to_string()
            } else {
                "light".to_string()
            }
        }
        Err(_) => "light".to_string(), // Fallback
    }
}
