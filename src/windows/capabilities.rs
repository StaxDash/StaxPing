// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

// Windows capability detection for StaxPing
// These checks only determine whether the OS *supports* the feature,
// not whether the current environment/firewall/user permissions allow it.

pub fn check_icmp_support() -> bool {
    // Windows always supports ICMP via IcmpSendEcho
    true
}

pub fn check_trace_support() -> bool {
    // Windows traceroute is implemented via ICMP TTL stepping
    true
}

pub fn check_dns_support() -> bool {
    // Windows supports DNSQuery_W for A/AAAA lookups
    true
}

pub fn check_http_support() -> bool {
    // reqwest works on Windows without special requirements
    true
}