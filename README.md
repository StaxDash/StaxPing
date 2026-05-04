[![Website](https://img.shields.io/badge/Website-zford.dev-000000?style=flat-square)](https://zford.dev)
[![itch.io](https://img.shields.io/badge/itch.io-StaxPing-FA5C5C?style=flat-square)](https://zforddev.itch.io/staxping)
[![Ko‑Fi](https://img.shields.io/badge/Support-KoFi-FF5E5B?style=flat-square)](https://ko-fi.com/zforddev)

---

# **StaxPing**
### **Lightweight, cross‑platform network diagnostics**

StaxPing is a clean, predictable replacement for the messy mix of `ping`, `dig`, `traceroute`, and `curl`.

Built in Rust for speed, safety, and portability — StaxPing provides a unified view of DNS resolution, ICMP latency, HTTP health, and optional hop‑by‑hop routing, all with consistent, readable output.

---

## **Features**

- **DNS Resolution**  
  Fast lookup with IPv4/IPv6 results and timing.

- **ICMP Ping**  
  Min/avg/max latency, packet loss, and jitter‑friendly timing.

- **HTTP Health Check**  
  Status code, response time, and final URL after redirects.

- **Optional Traceroute**  
  Hop‑by‑hop routing with aligned, readable output.

- **First‑Run Setup**  
  - EULA acceptance  
  - OS detection  
  - Capability checks  
  - Config stored in the user’s home directory  

After the first run, StaxPing works instantly with no prompts.

---

## **Usage**

Basic connectivity check:

```bash
staxping google.com
```

Include traceroute:

```bash
staxping google.com --trace
```

Advanced mode (reserved for future expansion):

```bash
staxping google.com --advanced
```

Help:

```bash
staxping --help
```

---

## **Example Output**

```bash
~ ❯ staxping example.com
========================================
  StaxPing v0.1.0 — Network Diagnostics
  Target: example.com
========================================

=== DNS ===============================
  IPv4:        ["93.184.216.34"]
  IPv6:        ["2606:2800:220:1:248:1893:25c8:1946"]
  Lookup:      20 ms

=== Ping ==============================
  Sent:        4
  Received:    4
  Loss:        0.0%
  Min:         20.10 ms
  Avg:         22.45 ms
  Max:         24.89 ms

=== HTTP ==============================
  Status:      200
  Time:        380 ms
  Final URL:   https://example.com/
```

---

## **Installation**

### **Linux (Primary Target)**  
A `.deb` package is available for Debian/Ubuntu‑based systems.

```bash
wget https://github.com/ZFordDev/StaxPing/releases/download/V0.1.0/staxping_0.1.0_amd64.deb
sudo dpkg -i staxping_0.1.0_amd64.deb
```

### **Windows**  
A standalone `.exe` is available in the releases section.  
PATH handling has been fixed for smoother setup.

---

## **Config & First‑Run Behavior**

On first run, StaxPing will:

1. Display the EULA  
2. Detect your OS  
3. Check system capabilities  
4. Store a small config file:

Linux:
```
$HOME/.config/staxping/config.json
```

Windows:
```
%APPDATA%\StaxPing\config.json
```

After that, StaxPing runs without prompts.

---

## **Built With**

- **Rust** — safety, speed, portability  
- `trust-dns-resolver` — DNS resolution  
- `surge-ping` — ICMP ping  
- `reqwest` — HTTP checks  
- `tracert` — traceroute  
- `clap` — CLI argument parsing  
- `serde` — config handling  

---

# **Roadmap**

StaxPing is evolving into a full network‑health companion.  
The next major milestone is **v0.2.0**, introducing persistent monitoring and local‑network awareness.

### **v0.2.0 (In Development)**  
- Local IPv4 detection (e.g., `192.168.0.10`)  
- Monitor mode — continuous checks with outage detection  
- HUD mode — always‑on status indicator (green/yellow/red)  
- Windows DNS performance improvements  
- UI polish and consistency updates  

### **Future Milestones**  
- Smart Diagnostics  
  - Rule‑based suggestions for likely network issues  
- Dash Mode  
  - Host + node architecture for multi‑location monitoring  
- Configurable DNS servers  
- Exportable logs and summaries  

---

## **Project Status**

StaxPing is **actively maintained** and growing.  
The current release includes:

- First‑run logic  
- Config system  
- Capability detection  
- DNS, ICMP, and HTTP modules  
- Optional traceroute  
- Polished CLI output  
- Linux `.deb` packaging  
- Windows `.exe` with PATH support  

The next release (0.3.0) focuses on monitoring, HUD mode, and local network awareness.

---

## **License**

StaxPing is **source‑available and noncommercial**.  
You’re free to view, study, modify, and redistribute the source code for personal or internal use.

Commercial use requires explicit written permission.

### Why the EULA Exists  
StaxPing includes a simple first‑run EULA — not as a barrier, but as part of the project’s learning and design goals.  
It demonstrates how to:

- present a license agreement on first launch  
- store user acceptance in a config file  
- build a clean, professional onboarding flow  

The EULA does **not** restrict features or limit general use.  
StaxPing will remain fully available, feature‑rich, and free for the community.

See `LICENSE` and `EULA.txt` for full terms.

---

## Explore More

**zford.dev** — projects, tools, and experiments.  
**itch.io** — downloadable builds and releases.  
**Ko‑Fi** — support ongoing development:  
https://ko-fi.com/zforddev

---
