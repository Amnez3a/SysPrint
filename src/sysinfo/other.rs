use crate::sysinfo::combine::DisplayOptions;
use chrono::Local;
use colored::ColoredString;
use colored::Colorize;
use std::env;
use std::fmt::Write;
use std::fs;

pub fn other_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) {
    // --- Other Info ---
    if !opts.other {
        return;
    }

    let _ = writeln!(buf, "{}", "--- Other Info ---".bold().cyan());

    // give link functions de_wm_check for variables lines
    de_wm_check(buf, c);

    // Functions de_wm_check
    fn de_wm_check(buf: &mut String, c: fn(&str) -> ColoredString) {
        // Desktop Environment / Window Manager
        let desktop = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string());
        let _ = writeln!(buf, "{}: {}", c("DE"), desktop);
    }

    fn wm_check(buf: &mut String, c: fn(&str) -> ColoredString) {
        let wm = if cfg!(target_os = "windows") {
            if env::var("GLAZEWM_VERSION").is_ok() {
                "GlazeWM".to_string()
            } else if env::var("KOMOREBI_CONFIG_HOME").is_ok() {
                "Komorebi".to_string()
            } else {
                "Explorer.exe".to_string()
            }
        } else if cfg!(target_os = "macos") {
            if env::var("YABAI_SOCKET").is_ok() {
                "Yabai".to_string()
            } else if env::var("AMETHYST_VERSION").is_ok() {
                "Amethyst".to_string()
            } else {
                "Quartz Compositor".to_string()
            }
        } else {
            // Linux / FreeBSD / OpenBSD
            if env::var("NIRI_SOCKET").is_ok() {
                "Niri".to_string()
            } else if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
                "Hyprland".to_string()
            } else if env::var("I3SOCK").is_ok() {
                "i3".to_string()
            } else if env::var("SWAYSOCK").is_ok() {
                "Sway".to_string()
            } else if env::var("BSPWM_SOCKET").is_ok() {
                "bspwm".to_string()
            } else if env::var("HERBSTLUFTWM_SOCKET").is_ok() {
                "herbstluftwm".to_string()
            } else if let Ok(wm_env) = env::var("WINDOWMANAGER") {
                wm_env.split('/').last().unwrap_or(&wm_env).to_string()
            } else {
                let desktop = env::var("XDG_CURRENT_DESKTOP")
                    .or_else(|_| env::var("DESKTOP_SESSION"))
                    .unwrap_or_default()
                    .to_lowercase();

                if desktop.contains("niri") {
                    "niri".to_string()
                } else if desktop.contains("kde") || env::var("KDE_FULL_SESSION").is_ok() {
                    "KWin".to_string()
                } else if desktop.contains("gnome") {
                    "Mutter".to_string()
                } else if desktop.contains("xfce") {
                    "Xfwm4".to_string()
                } else if desktop.contains("cinnamon") {
                    "Muffin".to_string()
                } else if desktop.contains("mate") {
                    "Marco".to_string()
                } else if !desktop.is_empty() {
                    desktop
                } else {
                    "Unknown".to_string()
                }
            }
        };

        // Protocol (Wayland / X11 / tty)
        let wm_display = if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
            // Register: "wayland" -> "Wayland", "x11" -> "X11"
            let session_lower = session_type.to_lowercase();
            let protocol = match session_lower.as_str() {
                "wayland" => "Wayland",
                "x11" => "X11",
                "tty" => "TTY",
                other => other,
            };
            format!("{} ({})", wm, protocol)
        } else {
            wm
        };

        let _ = writeln!(buf, "{}: {}", c("WM"), wm_display);
    }

    wm_check(buf, c);

    fn get_shell(buf: &mut String, c: fn(&str) -> ColoredString) {
        let shell_name = if let Ok(shell_path) = env::var("SHELL") {
            shell_path
                .split('/')
                .last()
                .unwrap_or("Unknown")
                .to_string()
        } else {
            env::var("ComSpec")
                .map(|p| p.split('\\').last().unwrap_or("cmd.exe").to_string())
                .unwrap_or_else(|_| "Unknown".to_string())
        };

        let _ = writeln!(buf, "{}: {}", c("Shell"), shell_name);
    }

    pub fn terminal_info(buf: &mut String, c: fn(&str) -> ColoredString) {
        let term = if let Ok(term) = env::var("TERM_PROGRAM") {
            if !term.is_empty() {
                term
            } else {
                get_fallback_term()
            }
        } else {
            get_fallback_term()
        };

        let _ = writeln!(buf, "{}: {}", c("Terminal"), term);
    }

    fn get_fallback_term() -> String {
        if env::var("KITTY_WINDOW_ID").is_ok() {
            return "kitty".to_string();
        }
        if env::var("ALACRITTY_SOCKET").is_ok() || env::var("ALACRITTY_LOG").is_ok() {
            return "alacritty".to_string();
        }
        if env::var("KONSOLE_VERSION").is_ok() {
            return "konsole".to_string();
        }
        if env::var("FOOT_SOCKET").is_ok() {
            return "foot".to_string();
        }
        if env::var("WT_SESSION").is_ok() {
            return "Windows Terminal".to_string();
        }
        if let Ok(term) = env::var("TERMINAL") {
            if !term.is_empty() {
                return term;
            }
        }
        if let Ok(term) = env::var("TERM") {
            if term != "xterm-256color" && !term.is_empty() {
                return term;
            }
        }

        "Unknown".to_string()
    }

    terminal_info(buf, c);
    get_shell(buf, c);
    sysprint(buf, c);

    fn system_time(buf: &mut String, c: fn(&str) -> ColoredString) {
        let now = Local::now();
        now.format("%H:%M").to_string();
        let _ = writeln!(
            buf,
            "{}: {}",
            c("Locale Time"),
            now.format("%H:%M").to_string()
        );
    }

    fn sysprint(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(
            buf,
            "{}: SysPrint v{}",
            c("Fetch"),
            env!("CARGO_PKG_VERSION")
        );
    }

    // give link functions battery for variables lines
    battery(buf, c);

    // Functions battery
    fn battery(buf: &mut String, c: fn(&str) -> ColoredString) {
        let mut battery_str = String::new();

        // --- Linux ---
        #[cfg(target_os = "linux")]
        {
            let mut found = false;
            if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("BAT") {
                        let path = entry.path();
                        let cap = fs::read_to_string(path.join("capacity")).unwrap_or_default();
                        let stat = fs::read_to_string(path.join("status")).unwrap_or_default();

                        if !cap.trim().is_empty() {
                            let status_str = if stat.trim().is_empty() {
                                "".to_string()
                            } else {
                                format!(" [{}]", stat.trim())
                            };
                            battery_str = format!("{}%{}", cap.trim(), status_str);
                            found = true;
                            break;
                        }
                    }
                }
            }
            if !found {
                return;
            }
        }

        // --- FreeBSD ---
        #[cfg(target_os = "freebsd")]
        {
            use std::process::Command;
            if let Ok(out) = Command::new("sysctl")
                .args(["-n", "hw.acpi.battery.life"])
                .output()
            {
                let cap = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !cap.is_empty() && cap != "-1" {
                    battery_str = format!("{}%", cap);
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        // --- OpenBSD ---
        #[cfg(target_os = "openbsd")]
        {
            use std::process::Command;
            if let Ok(out) = Command::new("apm").arg("-l").output() {
                let cap = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !cap.is_empty() && cap != "255" {
                    battery_str = format!("{}%", cap);
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        // --- NetBSD ---
        #[cfg(target_os = "netbsd")]
        {
            use std::process::Command;
            if let Ok(out) = Command::new("envstat").args(["-s", "bat0:charge"]).output() {
                let text = String::from_utf8_lossy(&out.stdout);
                if let Some(line) = text.lines().find(|l| l.contains('%')) {
                    if let Some(val) = line.split('(').next() {
                        let cleaned = val.replace("charge:", "").replace('%', "");
                        let trimmed = cleaned.trim();
                        if !trimmed.is_empty() {
                            battery_str = format!("{}%", trimmed);
                        } else {
                            return;
                        }
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        // --- Windows ---
        #[cfg(windows)]
        {
            use std::process::Command;
            if let Ok(out) = Command::new("wmic")
                .args(["path", "Win32_Battery", "get", "EstimatedChargeRemaining"])
                .output()
            {
                let text = String::from_utf8_lossy(&out.stdout);
                if let Some(cap) = text.lines().nth(1) {
                    let trimmed = cap.trim();
                    if !trimmed.is_empty() {
                        battery_str = format!("{}%", trimmed);
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        if !battery_str.is_empty() {
            let _ = writeln!(buf, "{}: {}", c("Battery"), battery_str);
        }
    }
    system_time(buf, c);
}
