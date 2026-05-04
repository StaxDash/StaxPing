// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

// --- Platform Routing --------------------------------------------------------

mod linux;
mod windows;
mod shared;

#[cfg(target_os = "linux")]
use linux as platform;

#[cfg(target_os = "windows")]
use windows as platform;

// --- Shared Modules ----------------------------------------------------------

use shared::config::Config;
use shared::first_run::run_first_run;
use shared::http;
use shared::localnet;


// --- CLI ---------------------------------------------------------------------

use clap::Parser;

/// Simple aligned key/value printer
fn kv(label: &str, value: impl std::fmt::Display) {
    println!("  {:<12} {}", label, value);
}

/// CLI argument structure
#[derive(Parser, Debug)]
#[command(
    name = "staxping",
    version = "0.2.1",
    about = "",
    override_usage = "staxping <target> [options]",
    help_template = "USAGE:
  {usage}

EXAMPLES:
  staxping google.com
  staxping example.com --trace
  staxping 8.8.8.8

OPTIONS:
{options}
"
)]
struct Cli {
    /// Target domain or IP address
    target: Option<String>,

    /// Run a hop-by-hop traceroute after ping
    #[arg(long)]
    trace: bool,

    /// Reserved for future diagnostic features
    #[arg(short = 'A', long)]
    advanced: bool,

    /// Display local network information
    #[arg(long)]
    localnet: bool,
}

// --- Main --------------------------------------------------------------------

#[tokio::main]
async fn main() {
    // Load config
    let config = Config::load();

    match config {
        Some(cfg) => {
            if cfg.needs_first_run() {
                run_initial_setup();
                return;
            }
        }
        None => {
            run_initial_setup();
            return;
        }
    }

    // Parse CLI arguments
    let cli = Cli::parse();

    // Handle --localnet mode (skip everything else)
    if cli.localnet {
        println!("========================================");
        println!("  StaxPing v0.2.1 — Local Network Info");
        println!("========================================\n");

        let local_info = localnet::get_local_info();

        println!("=== Local Network =====================");
        kv("IPv4:", local_info.ipv4.unwrap_or("unknown".into()));
        kv("Interface:", local_info.interface.unwrap_or("unknown".into()));
        kv("Gateway:", local_info.gateway.unwrap_or("unknown".into()));

        return;
    }

    // If no target provided -> show friendly hint
    if cli.target.is_none() {
        println!("StaxPing needs a target to run diagnostics.\n");
        println!("Try:");
        println!("  staxping example.com");
        println!("  staxping example.com --trace");
        println!("  staxping --help");
        return;
    }

    // Extract target
    let target = cli.target.unwrap();

    // Top-level banner
    println!("========================================");
    println!("  StaxPing v0.2.1 — Network Diagnostics");
    println!("  Target: {}", target);
    println!("========================================\n");

    // --- DNS -----------------------------------------------------------------

    println!("=== DNS ===============================");

    let dns_result = match platform::dns::resolve_domain(&target).await {
        Ok(result) => {
            if !result.ipv4.is_empty() {
                kv("IPv4:", format!("{:?}", result.ipv4));
            }
            if !result.ipv6.is_empty() {
                kv("IPv6:", format!("{:?}", result.ipv6));
            }
            kv("Lookup:", format!("{} ms", result.lookup_ms));
            result
        }
        Err(e) => {
            kv("DNS error:", e);
            return;
        }
    };

    // Use the first IPv4 address for ping
    let ping_ip = if !dns_result.ipv4.is_empty() {
        dns_result.ipv4[0].clone()
    } else if !dns_result.ipv6.is_empty() {
        dns_result.ipv6[0].clone()
    } else {
        println!("No valid IPs found for ping.");
        return;
    };

    // --- Ping ----------------------------------------------------------------

    println!("\n=== Ping ==============================");

    match platform::ping::run_ping(&ping_ip).await {
        Ok(result) => {
            kv("Sent:", result.sent);
            kv("Received:", result.received);
            kv("Loss:", format!("{:.1}%", result.loss));
            kv("Min:", format!("{:.2} ms", result.min_ms));
            kv("Avg:", format!("{:.2} ms", result.avg_ms));
            kv("Max:", format!("{:.2} ms", result.max_ms));
        }
        Err(e) => {
            kv("Ping error:", e);
        }
    }

    // --- HTTP ----------------------------------------------------------------

    println!("\n=== HTTP ==============================");

    match http::check_http(&target).await {
        Ok(result) => {
            kv("Status:", result.status);
            kv("Time:", format!("{} ms", result.time_ms));
            kv("Final URL:", result.final_url);
        }
        Err(e) => {
            kv("HTTP error:", e);
        }
    }

    // --- Traceroute ----------------------------------------------------------

    if cli.trace {
        println!("\n=== Traceroute ========================");

        let trace_ip = if !dns_result.ipv4.is_empty() {
            dns_result.ipv4[0].clone()
        } else if !dns_result.ipv6.is_empty() {
            dns_result.ipv6[0].clone()
        } else {
            println!("No valid IPs found for traceroute.");
            return;
        };

        match platform::trace::run_trace(&trace_ip).await {
            Ok(result) => {
                for hop in result.hops {
                    let times: Vec<String> = hop.times_ms.iter()
                        .map(|t| format!("{:.2} ms", t))
                        .collect();

                    println!(
                        "  {:>2}  {:<15}  {}",
                        hop.hop,
                        hop.ip,
                        times.join("  ")
                    );
                }
            }
            Err(e) => {
                kv("Trace error:", e);
            }
        }
    }

    if cli.advanced {
        println!("\n(advanced mode enabled)");
    }
}

// --- First Run Wrapper -------------------------------------------------------

fn run_initial_setup() {
    let os = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "unknown"
    };

    let icmp = platform::capabilities::check_icmp_support();
    let trace = platform::capabilities::check_trace_support();
    let dns = platform::capabilities::check_dns_support();
    let http = platform::capabilities::check_http_support();

    run_first_run(os, icmp, trace, dns, http);
}