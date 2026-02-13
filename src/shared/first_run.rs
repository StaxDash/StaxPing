// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

use std::io::{self, Write};
use super::config::Config;

/// Runs the first‑run setup flow.
/// Capability flags are passed in from main.rs (platform‑specific).
pub fn run_first_run(
    os: &str,
    icmp: bool,
    trace: bool,
    dns: bool,
    http: bool,
) {
    println!("=== StaxPing ===");
    println!("a product of StaxDash | life made simple\n");

    show_eula();
    if !prompt_acceptance() {
        println!("EULA not accepted. Exiting StaxPing.");
        std::process::exit(0);
    }

    println!("\nPerforming initial setup...");

    println!("• Detecting OS: {}", os);
    println!("• Checking ICMP support: {}", yes_no(icmp));
    println!("• Checking traceroute capability: {}", yes_no(trace));
    println!("• Checking DNS resolver: {}", yes_no(dns));
    println!("• Checking HTTP client: {}", yes_no(http));

    let config = Config::new_after_first_run(os, icmp, trace, dns, http);

    if let Err(e) = config.save() {
        println!("Failed to save config: {}", e);
        std::process::exit(1);
    }

    println!("\nSetup complete. You can now use StaxPing normally.");
}

/// Display the EULA text (loaded from repo root)
fn show_eula() {
    // Adjusted path: shared/first_run.rs → ../../EULA.txt
    let eula_text = include_str!("../../EULA.txt");
    println!("{}", eula_text);
}

/// Ask the user to type "yes" to accept the EULA
fn prompt_acceptance() -> bool {
    print!("Type 'yes' to accept: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().eq_ignore_ascii_case("yes")
}

/// Helper for printing Yes/No
fn yes_no(value: bool) -> &'static str {
    if value { "OK" } else { "Unavailable" }
}