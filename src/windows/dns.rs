// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

// Windows DNS resolver using Win32 API

use tokio::time::{timeout, Duration};
use trust_dns_resolver::{
    TokioAsyncResolver,
    name_server::{GenericConnector, TokioRuntimeProvider},
};
use std::time::Instant;

pub struct DnsResult {
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
    pub lookup_ms: u128,
}

pub async fn resolve_domain(domain: &str) -> Result<DnsResult, String> {
    // Build the required connector
    let connector = GenericConnector::new(TokioRuntimeProvider::default());

    // Construct resolver from system configuration
    let resolver = TokioAsyncResolver::from_system_conf(connector)
        .map_err(|e| format!("Resolver init failed: {}", e))?;

    let start = Instant::now();
    let dns_timeout = Duration::from_secs(3);

    // A records
    let ipv4_lookup = timeout(dns_timeout, resolver.ipv4_lookup(domain)).await
        .map_err(|_| "DNS lookup timed out after 3 seconds".to_string())?
        .map_err(|e| format!("DNS A lookup failed: {}", e))?;
    let ipv4: Vec<String> = ipv4_lookup.iter().map(|ip| ip.to_string()).collect();

    // AAAA records
    let ipv6_lookup = timeout(dns_timeout, resolver.ipv6_lookup(domain)).await
        .map_err(|_| "DNS lookup timed out after 3 seconds".to_string())?
        .map_err(|e| format!("DNS AAAA lookup failed: {}", e))?;
    let ipv6: Vec<String> = ipv6_lookup.iter().map(|ip| ip.to_string()).collect();

    let elapsed = start.elapsed().as_millis();

    Ok(DnsResult {
        ipv4,
        ipv6,
        lookup_ms: elapsed,
    })
}