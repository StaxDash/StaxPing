// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

// Windows-native ICMP ping implementation
use windows::Win32::NetworkManagement::IpHelper::{
    IcmpCreateFile, IcmpSendEcho, IcmpCloseHandle, ICMP_ECHO_REPLY, IP_OPTION_INFORMATION,
};
use windows::Win32::Foundation::HANDLE;
use std::ffi::c_void;
use std::mem::size_of;
use std::time::{Duration, Instant};
use tokio::task::spawn_blocking;
use tokio::time::timeout;

pub struct PingResult {
    pub sent: u32,
    pub received: u32,
    pub loss: f32,
    pub min_ms: f64,
    pub avg_ms: f64,
    pub max_ms: f64,
}

pub async fn run_ping(ip: &str) -> Result<PingResult, String> {
    // Create ICMP handle
    let handle = unsafe { IcmpCreateFile() }
        .map_err(|e| format!("Failed to create ICMP handle: {}", e))?;

    if handle.is_invalid() {
        return Err("ICMP handle is invalid".into());
    }

    let sent = 4;
    let mut received = 0;
    let mut times = Vec::new();

    for _ in 0..sent {
        let start = Instant::now();
        let ip_string = ip.to_string();

        let send_future = spawn_blocking(move || send_single_ping(handle, &ip_string));
        let send_result = timeout(Duration::from_secs(3), send_future).await
            .map_err(|_| "ICMP ping timed out".to_string())?
            .map_err(|e| format!("Ping task failed: {}", e))?;

        match send_result {
            Ok(true) => {
                received += 1;
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                times.push(elapsed);
            }
            Ok(false) => {
                // timeout or unreachable
            }
            Err(e) => {
                unsafe { IcmpCloseHandle(handle).ok(); }
                return Err(e);
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    unsafe { IcmpCloseHandle(handle).ok(); }

    Ok(calculate_stats(sent, received, times))
}

fn send_single_ping(handle: HANDLE, ip: &str) -> Result<bool, String> {
    let ipv4 = ip
        .parse::<std::net::Ipv4Addr>()
        .map_err(|_| format!("Invalid IPv4 address: {}", ip))?;

    let addr = u32::from_ne_bytes(ipv4.octets());

    let payload = b"staxping";
    let mut reply_buf = vec![0u8; size_of::<ICMP_ECHO_REPLY>() + payload.len() + 8];

    // Default TTL = 128
    let mut options = IP_OPTION_INFORMATION {
        Ttl: 128,
        ..Default::default()
    };

    let result = unsafe {
        IcmpSendEcho(
            handle,
            addr,
            payload.as_ptr() as *const c_void,
            payload.len() as u16,
            Some(&mut options),
            reply_buf.as_mut_ptr() as *mut c_void,
            reply_buf.len() as u32,
            2000, // timeout ms
        )
    };

    if result == 0 {
        return Ok(false);
    }

    Ok(true)
}

fn calculate_stats(sent: u32, received: u32, times: Vec<f64>) -> PingResult {
    let loss = ((sent - received) as f32 / sent as f32) * 100.0;

    let (min_ms, avg_ms, max_ms) = if !times.is_empty() {
        let min = times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let avg = times.iter().sum::<f64>() / times.len() as f64;
        (min, avg, max)
    } else {
        (0.0, 0.0, 0.0)
    };

    PingResult {
        sent,
        received,
        loss,
        min_ms,
        avg_ms,
        max_ms,
    }
}