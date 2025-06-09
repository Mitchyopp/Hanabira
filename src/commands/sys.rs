use serenity::model::channel::Message;
use serenity::prelude::Context;
use sysinfo::{
    System, RefreshKind, CpuRefreshKind, MemoryRefreshKind,
};

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    let mut parts = Vec::new();
    if days > 0 { parts.push(format!("{}d", days)); }
    if hours > 0 { parts.push(format!("{}h", hours)); }
    if minutes > 0 { parts.push(format!("{}m", minutes)); }
    parts.push(format!("{}s", secs));

    parts.join(" ")
}

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.content == "!sys" {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_memory(MemoryRefreshKind::everything())
                .with_cpu(CpuRefreshKind::everything()),
        );

        sys.refresh_memory();
        sys.refresh_cpu();

        let hostname = System::host_name().unwrap_or_else(|| "Unknown".into());
        let os_name = System::name().unwrap_or_else(|| "Unknown".into());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".into());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".into());
        let uptime = format_uptime(System::uptime());
        let total_memory = sys.total_memory() / 1024;
        let used_memory = sys.used_memory() / 1024;
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let cpu_cores = sys.cpus().len();

        let response = format!(
            "**System Info:**\n\
            • Hostname: `{}`\n\
            • OS: `{} {}`\n\
            • Kernel: `{}`\n\
            • Uptime: `{}`\n\
            • CPU Cores: `{}`\n\
            • CPU Usage: `{:.2}%`\n\
            • Memory: `{}/{} MB`",
            hostname,
            os_name,
            os_version,
            kernel_version,
            uptime,
            cpu_cores,
            cpu_usage,
            used_memory,
            total_memory
        );

        let _ = msg.channel_id.say(&ctx.http, response).await;
    }
}

