# **StaxPing Classic — Legacy Network Diagnostics Tool**

<p align="center">
  <img src="https://img.shields.io/github/v/release/zforddev/staxping?style=for-the-badge&color=78C2AD" alt="Version">
  <img src="https://img.shields.io/github/license/zforddev/staxping?style=for-the-badge&color=blue" alt="License">
  <img src="https://img.shields.io/github/stars/zforddev/staxping?style=for-the-badge&color=FFD700" alt="Stars">
</p>

**StaxPing Classic** is the original version of StaxPing — a lightweight, cross‑platform network diagnostics tool built in Rust.  
This branch is now preserved as a **legacy edition** while the next‑generation Stax ecosystem is being developed.

The classic version remains available for historical reference, stability, and users who prefer the original workflow.

---

## 💡 Why This Branch Exists

StaxPing began as a simple, unified replacement for tools like:

- `ping`  
- `dig`  
- `traceroute`  
- `curl`  

It provided a clean, predictable CLI for DNS, ICMP, HTTP, and routing checks — all in one binary.

As the project grew, StaxPing started expanding into system monitoring, uptime tracking, and resource inspection.  
To avoid becoming bloated, the project is now being **split into multiple focused tools**:

- **StaxPing** — network diagnostics  
- **StaxSpec** — system specs & resource monitoring  
- **StaxDash** — unified GUI dashboard (paid convenience layer)

This branch preserves the original tool before the ecosystem reboot.

---

## ✨ Philosophy (Classic Edition)

The classic version of StaxPing was built on a few core ideas:

- **Speed & Safety** — Rust‑powered performance  
- **Predictability** — consistent, readable output  
- **Cross‑Platform** — Linux, Windows, macOS  
- **Zero Bloat** — one binary, one job  
- **Professional Onboarding** — EULA, config, capability checks  

These principles continue in the new ecosystem, but with a cleaner, modular architecture.

---

## 🛠 Current State (Classic)

| Version | Status | Focus |
|--------|--------|--------|
| **StaxPing Classic (v0.x)** | `Legacy` | Original all‑in‑one network tool |
| **StaxPing 2.x** | `In Dev` | Modular rewrite with updater + ecosystem support |
| **Stax Ecosystem** | `Planned` | StaxPing + StaxSpec + StaxDash |

---

## 🌐 Part of a Larger Ecosystem

The classic version was the seed of what is now becoming the **Stax SysAdmin Toolkit** — a suite of small, sharp, local‑first tools with a premium GUI layer.

The new architecture lives on the `main` branch and in separate repos:

- `staxping` (network diagnostics)  
- `staxspec` (system specs)  
- `staxdash` (GUI dashboard)  
- `stax` (central registry + ecosystem docs)

This branch remains frozen as a reference point.

---

## 🚀 Installation (Classic)

### Linux (.deb)
```bash
wget https://github.com/ZFordDev/StaxPing/releases/download/V0.1.0/staxping_0.1.0_amd64.deb
sudo dpkg -i staxping_0.1.0_amd64.deb
```

### Windows (.exe)
Download from the Releases page.  
PATH handling is automatic on first run.

---

## 📖 Documentation & Links

**Classic Resources:**
- `config.json` (first‑run setup)
- DNS, ICMP, HTTP, traceroute modules
- EULA onboarding flow

**New Ecosystem:**
- StaxPing 2.x (main branch)
- StaxSpec (system monitoring)
- StaxDash (GUI dashboard)
- Central registry (`stax.toml`)

---

## 👥 Credits

Created and maintained by **ZFordDev**.  
Thanks to early users who shaped the original tool and inspired the ecosystem reboot.

---

## ❤️ Support the Project

If StaxPing Classic helped you diagnose a network issue or learn Rust CLI design:

- ⭐ **Star the repo** to help others discover it  
- ☕ **Support on Ko‑Fi**: [https://ko-fi.com/zforddev](https://ko-fi.com/zforddev)  
