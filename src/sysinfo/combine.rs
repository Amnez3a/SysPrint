use crate::sysinfo::cpu::cpu_info;
use crate::sysinfo::disks::disk_info;
use crate::sysinfo::gpu::get_gpu_info;
use crate::sysinfo::memory::memory_info;
use crate::sysinfo::other::other_info;
use crate::sysinfo::system::system_info;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Clone, Copy)]
pub struct DisplayOptions {
    pub system: bool,
    pub cpu: bool,
    pub memory: bool,
    pub disks: bool,
    pub other: bool,
    pub gpu: bool,
    pub mini_logo_mode: bool,
    pub compact_mode: bool,
    pub fast_mode: bool,
    pub hide_fetch_info: bool,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            system: true,
            cpu: true,
            memory: true,
            disks: true,
            other: true,
            gpu: true,
            mini_logo_mode: false,
            compact_mode: false,
            fast_mode: false,
            hide_fetch_info: false,
        }
    }
}

pub struct SystemInfo {
    pub buffer: String,
    pub mini_logo_mode: bool,
}

impl SystemInfo {
    pub fn collect(opts: DisplayOptions) -> Self {
        let mut buffer = String::with_capacity(2048);
        let (_, _, c) = crate::logos::get_logo(opts.mini_logo_mode);

        if opts.compact_mode {
            let _sys = System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            );

            system_info(&opts, &mut buffer, c);
            cpu_info(&opts, &mut buffer, &_sys, c);
            get_gpu_info(&opts, &mut buffer, opts.fast_mode, c);
            memory_info(&opts, &mut buffer, &_sys, c);
            other_info(&opts, &mut buffer, c);

            return Self {
                buffer,
                mini_logo_mode: opts.mini_logo_mode,
            };
        }

        // --- STANDARD / FULL MODE BRANCH ---
        let _sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );

        if opts.system {
            system_info(&opts, &mut buffer, c);
        }
        if opts.cpu {
            cpu_info(&opts, &mut buffer, &_sys, c);
        }
        if opts.gpu {
            get_gpu_info(&opts, &mut buffer, opts.fast_mode, c);
        }
        if opts.memory {
            memory_info(&opts, &mut buffer, &_sys, c);
        }
        if opts.other {
            other_info(&opts, &mut buffer, c);
        }
        if opts.disks {
            disk_info(&opts, &mut buffer, c);
        }

        Self {
            buffer,
            mini_logo_mode: opts.mini_logo_mode,
        }
    }
}
