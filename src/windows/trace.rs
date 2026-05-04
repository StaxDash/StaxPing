// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

// Windows-native traceroute using ICMP Echo with TTL stepping

use windows::Win32::NetworkManagement::IpHelper::{
    IcmpCreateFile, IcmpSendEcho, IcmpCloseHandle, ICMP_ECHO_REPLY, IP_OPTION_INFORMATION,
};
use windows::Win32::Foundation::HANDLE;

use std::ffi::c_void;
use std::mem::size_of;
use std::time::{Duration, Instant};
use tokio::task::spawn_blocking;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct TraceHop {
    pub hop: u32,
    pub host: String,
    pub ip: String,
    pub times_ms: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct TraceResult {
    pub hops: Vec<TraceHop>,
}

pub async fn run_trace(target: &str) -> Result<TraceResult, String> {
    // Create ICMP handle
    let handle = unsafe { IcmpCreateFile() }
        .map_err(|e| format!("Failed to create ICMP handle: {}", e))?;

    if handle.is_invalid() {
        return Err("ICMP handle is invalid".into());
    }

    // Parse IPv4 address
    let ipv4 = target
        .parse::<std::net::Ipv4Addr>()
        .map_err(|_| format!("Invalid IPv4 address: {}", target))?;

    let dest_ip = u32::from_ne_bytes(ipv4.octets());

    let mut hops = Vec::new();
    let max_hops = 30;

    for ttl in 1..=max_hops {
        let send_future = spawn_blocking(move || send_trace_hop(handle, dest_ip, ttl));
        let hop = timeout(Duration::from_secs(3), send_future).await
            .map_err(|_| "Traceroute hop timed out".to_string())?
            .map_err(|e| format!("Trace task failed: {}", e))??;

        hops.push(hop.clone());

        // Stop if we reached the destination
        if hop.ip == target {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }

    unsafe { IcmpCloseHandle(handle).ok(); }

    Ok(TraceResult { hops })
}

fn send_trace_hop(handle: HANDLE, dest_ip: u32, ttl: u32) -> Result<TraceHop, String> {
    let payload = b"staxping";
    let mut reply_buf = vec![0u8; size_of::<ICMP_ECHO_REPLY>() + payload.len() + 8];

    // TTL stepping
    let mut options = IP_OPTION_INFORMATION {
        Ttl: ttl as u8,
        ..Default::default()
    };

    let start = Instant::now();

    let result = unsafe {
        IcmpSendEcho(
            handle,
            dest_ip,
            payload.as_ptr() as *const c_void,
            payload.len() as u16,
            Some(&mut options),
            reply_buf.as_mut_ptr() as *mut c_void,
            reply_buf.len() as u32,
            2000, // timeout
        )
    };

    let elapsed = start.elapsed().as_secs_f64() * 1000.0;

    if result == 0 {
        // timeout or unreachable
        return Ok(TraceHop {
            hop: ttl,
            host: "*".into(),
            ip: "*".into(),
            times_ms: vec![elapsed],
        });
    }

    // Extract reply
    let reply: &ICMP_ECHO_REPLY =
        unsafe { &*(reply_buf.as_ptr() as *const ICMP_ECHO_REPLY) };

    let addr = reply.Address.to_ne_bytes();
    let ip = format!("{}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3]);

    Ok(TraceHop {
        hop: ttl,
        host: ip.clone(),
        ip,
        times_ms: vec![elapsed],
    })
}