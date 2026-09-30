use sysinfo::System;
use colored::*;
use std::path::Path;
use std::env;

pub fn get_logo(mini_logo: bool) -> (Vec<ColoredString>, usize, fn(&str) -> ColoredString) {
    let mut os_name = System::name().unwrap_or_default().to_lowercase();

    // Android/Termux
    let is_android = Path::new("/system/build.prop").exists()
        || env::var("PREFIX").map(|p| p.contains("com.termux")).unwrap_or(false);

    if is_android {
        os_name = "android".to_string();
    }

    let folder = if mini_logo { "assets/mini" } else { "assets/normal" };

let (filename, color_func): (&str, fn(&str) -> ColoredString) = match os_name.to_lowercase() {
    // Arch Based
    s if s.contains("cachyos") => ("cachyos.txt", |s| s.green().bold()),
    s if s.contains("manjaro") => ("manjaro.txt", |s| s.green().bold()),
    s if s.contains("endeavour") => ("endeavour.txt", |s| s.purple().bold()),
    s if s.contains("artix") => ("artix.txt", |s| s.blue().bold()),
    s if s.contains("arch") => ("arch.txt", |s| s.blue().bold()),

    // Debian Based
    s if s.contains("kali") => ("kali.txt", |s| s.white().bold()),
    s if s.contains("astra") => ("astra.txt", |s| s.blue().bold()),
    s if s.contains("debian") => ("debian.txt", |s| s.red().bold()),

    // Ubuntu Based
    s if s.contains("mint") => ("mint.txt", |s| s.green().bold()),
    s if s.contains("zorinos") || s.contains("zorin") => ("zorin.txt", |s| s.blue().bold()),
    s if s.contains("pop") || s.contains("popos") => ("popos.txt", |s| s.blue().bold()),
    s if s.contains("lubuntu") => ("lubuntu.txt", |s| s.blue().bold()),
    s if s.contains("kubuntu") => ("kubuntu.txt", |s| s.blue().bold()),
    s if s.contains("xubuntu") => ("xubuntu.txt", |s| s.blue().bold()),
    s if s.contains("ubuntu") => ("ubuntu.txt", |s| s.red().bold()),

    // BSD
    s if s.contains("freebsd") => ("freebsd.txt", |s| s.red().bold()),
    s if s.contains("netbsd") => ("netbsd.txt", |s| s.yellow().bold()),
    s if s.contains("openbsd") => ("openbsd.txt", |s| s.yellow().bold()),

    // Independent
    s if s.contains("android") => ("android.txt", |s| s.green().bold()),
    s if s.contains("opensuse") => ("opensuse.txt", |s| s.green().bold()),
    s if s.contains("nixos") => ("nixos.txt", |s| s.blue().bold()),
    s if s.contains("void") => ("void.txt", |s| s.cyan().bold()),
    s if s.contains("windows") => ("windows.txt", |s| s.blue().bold()),
    s if s.contains("fedora") => ("fedora.txt", |s| s.blue().bold()),
    s if s.contains("gentoo") => ("gentoo.txt", |s| s.white().bold()),
    s if s.contains("alpine") => ("alpine.txt", |s| s.purple().bold()),
    s if s.contains("darwin") || s.contains("mac") => ("apple.txt", |s| s.white().bold()),

    // Default
    _ => ("tux.txt", |s| s.white()),    
};

let path = format!("{}/{}", folder, filename);
let raw_logo = std::fs::read_to_string(&path)
    .unwrap_or_else(|_| "Logo not found".to_string());
    
    let max_width = raw_logo.lines().map(|l| l.chars().count()).max().unwrap_or(0);

    let logo_lines = raw_logo
        .lines()
        .map(|line| {
            let char_count = line.chars().count();
            let padding = " ".repeat(max_width.saturating_sub(char_count));
            color_func(&format!("{line}{padding}"))
        })
        .collect();

    (logo_lines, max_width, color_func)
}
