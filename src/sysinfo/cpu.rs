use std::fmt::Write;
use colored::{ColoredString, Colorize};
use sysinfo::{Components, System};
use crate::sysinfo::combine::DisplayOptions;

pub fn cpu_info(opts: &DisplayOptions, buf: &mut String, sys: &System, c: fn(&str) -> ColoredString) {
    if !opts.cpu.enabled {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", "--- CPU INFO ---".bold().cyan());
    }

    if opts.cpu.name || opts.compact_mode {
        cpu_name(buf, sys, c);
    }

    if opts.compact_mode {
        return;
    }

    if opts.cpu.ghz {
        ggz_cpu(buf, sys, c);
    }
    if opts.cpu.usage {
        cpu_usage(buf, sys, c);
    }
    if opts.cpu.temp {
        cpu_temperature(buf, c);
    }
    if opts.cpu.cores || opts.cpu.threads {
        cpu_cores_and_threads(buf, sys, c, opts.cpu.cores, opts.cpu.threads);
    }
    if opts.cpu.architecture {
        cpu_arch(buf, c);
    }
}

fn cpu_name(buf: &mut String, sys: &System, c: fn(&str) -> ColoredString) {
    let cpus = sys.cpus();
    if let Some(cpu) = cpus.first() {
        let _ = writeln!(buf, "{}: {}", c("CPU name"), cpu.brand().trim());
    } else {
        let _ = writeln!(buf, "CPU: Unknown");
    }
}

fn ggz_cpu(buf: &mut String, sys: &System, c: fn(&str) -> ColoredString) {
    let cpus = sys.cpus();
    if let Some(cpu) = cpus.first() {
        let freq_ghz = cpu.frequency() as f64 / 1000.0;
        let _ = writeln!(buf, "{}: {:.2} GHz", c("GHz"), freq_ghz);
    }
}

fn cpu_usage(buf: &mut String, sys: &System, c: fn(&str) -> ColoredString) {
    let usage = sys.global_cpu_usage();
    if usage > 99.9 && cfg!(target_os = "windows") {
        let _ = writeln!(buf, "{}: N/A", c("CPU Usage"));
    } else {
        let _ = writeln!(buf, "{}: {:.1}%", c("CPU Usage"), usage);
    }
}

fn cpu_temperature(buf: &mut String, c: fn(&str) -> ColoredString) {
    let components = Components::new_with_refreshed_list();
    let cpu_temp = components.iter().find_map(|comp| {
        let label = comp.label().to_lowercase();
        if label.contains("cpu")
            || label.contains("core")
            || label.contains("package")
            || label.contains("k10temp")
            || label.contains("zenpower")
        {
            comp.temperature()
        } else {
            None
        }
    });

    if let Some(temp) = cpu_temp {
        let _ = writeln!(buf, "{}: {:.1}°C", c("CPU Temp"), temp);
    }
}

fn cpu_cores_and_threads(
    buf: &mut String,
    sys: &System,
    c: fn(&str) -> ColoredString,
    show_cores: bool,
    show_threads: bool,
) {
    if show_cores {
        let _ = writeln!(buf, "{}: {}", c("Cores"), System::physical_core_count().unwrap_or(0));
    }
    if show_threads {
        let cpus = sys.cpus();
        let _ = writeln!(buf, "{}: {}", c("Threads"), cpus.len());
    }
}

fn cpu_arch(buf: &mut String, c: fn(&str) -> ColoredString) {
    let _ = writeln!(buf, "{}: {}", c("Architecture"), std::env::consts::ARCH);
}