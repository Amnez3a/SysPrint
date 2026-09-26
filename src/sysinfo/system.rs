use crate::sysinfo::combine::DisplayOptions;
use colored::ColoredString;
use colored::Colorize;
use std::env;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use sysinfo::System;

// Uptime functions
fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

// --- SYSTEM INFO ---
pub fn system_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) {
    if !opts.system {
        return;
    }

    let _ = writeln!(buf, "{}", "--- System INFO ---".bold().cyan());

    os_name(buf, c);

    // OS_name
    fn os_name(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(buf, "{}: {}", c("OS"), System::name().unwrap_or_default());
    }

    os_version(buf, c);
    check_kernel(buf, c);
    init_info(buf, c);

    // OS_version
    fn os_version(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(
            buf,
            "{}: {}",
            c("OS Version"),
            System::os_version().unwrap_or_default()
        );
    }

    host(buf, c);
    user_info(buf, c);

    // Host name
    fn host(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(
            buf,
            "{}: {}",
            c("Host"),
            System::host_name().unwrap_or_default()
        );
    }

    fn check_kernel(buf: &mut String, c: fn(&str) -> ColoredString) {
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

        let _ = writeln!(buf, "{}: {}", c("Kernel"), kernel);
    }

    pub fn get_init_system() -> String {
        if let Ok(comm) = fs::read_to_string("/proc/1/comm") {
            let name = comm.trim();
            match name {
                "systemd" => return "systemd".to_string(),
                "init" | "sysvinit" => {
                    if Path::new("/run/openrc").exists() || Path::new("/etc/openrc").exists() {
                        return "OpenRC".to_string();
                    }
                    if Path::new("/etc/runit").exists() || Path::new("/run/runit").exists() {
                        return "runit".to_string();
                    }
                    return "SysVinit".to_string();
                }
                "runit" | "runit-init" => return "runit".to_string(),
                "dinit" => return "dinit".to_string(),
                "s6-svscan" => return "s6".to_string(),
                _ => return name.to_string(),
            }
        }

        if Path::new("/run/systemd/system").exists() {
            "systemd".to_string()
        } else if Path::new("/run/openrc").exists() || Path::new("/etc/openrc").exists() {
            "OpenRC".to_string()
        } else if Path::new("/etc/runit").exists() || Path::new("/run/runit").exists() {
            "runit".to_string()
        } else if Path::new("/sbin/dinit").exists() {
            "dinit".to_string()
        } else if cfg!(target_os = "freebsd")
            || cfg!(target_os = "openbsd")
            || cfg!(target_os = "netbsd")
        {
            "BSD init".to_string()
        } else if cfg!(target_os = "macos") {
            "launchd".to_string()
        } else if cfg!(windows) {
            "SMSS".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    pub fn init_info(buf: &mut String, c: fn(&str) -> ColoredString) {
        let init = get_init_system();
        let _ = writeln!(buf, "{}: {}", c("Init"), init);
    }

    pub fn user_info(buf: &mut String, c: fn(&str) -> ColoredString) {
        let username = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        let _ = writeln!(buf, "{}: {}", c("User"), username);
    }

    //Uptime
    let _ = writeln!(buf, "{}: {}", c("Uptime"), format_uptime(System::uptime()));
}
