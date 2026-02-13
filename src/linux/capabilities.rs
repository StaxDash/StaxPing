// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

pub fn check_icmp_support() -> bool {
    true // Linux always supports ICMP if permissions allow
}

pub fn check_trace_support() -> bool {
    true
}

pub fn check_dns_support() -> bool {
    true
}

pub fn check_http_support() -> bool {
    true
}