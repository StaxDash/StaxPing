# **StaxPing 2.x — Network Diagnostics, Reimagined**

<p align="center">
  <img src="https://img.shields.io/github/v/release/zforddev/staxping?style=for-the-badge&color=78C2AD" alt="Version">
  <img src="https://img.shields.io/github/license/zforddev/staxping?style=for-the-badge&color=blue" alt="License">
  <img src="https://img.shields.io/github/stars/zforddev/staxping?style=for-the-badge&color=FFD700" alt="Stars">
</p>

StaxPing is entering its next chapter — rebuilt from the ground up as part of a **modular, modern sysadmin toolkit**.  
This branch represents the **new StaxPing 2.x foundation**, focused on clarity, modularity, and long‑term maintainability.

The original version of StaxPing is now preserved on the `classic` branch.

---

## 💡 Why StaxPing Is Being Rebuilt

The original StaxPing grew quickly — DNS, ICMP, HTTP, traceroute, monitoring ideas, system info, uptime checks.  
It was becoming powerful, but also **too broad for a single binary**.

To avoid bloat and to support real sysadmin workflows, StaxPing is now part of a **three‑tool ecosystem**:

- **StaxPing** — network diagnostics  
- **StaxSpec** — system specs & resource monitoring  
- **StaxDash** — unified GUI dashboard (paid convenience layer)

Each tool is focused, fast, and built for reliability.  
Together, they form a clean, cohesive toolkit.

---

## ✨ Philosophy

The Stax 2.x ecosystem is built on a few core principles:

- **Local‑first** — everything runs on your machine  
- **Zero bloat** — each tool does one job extremely well  
- **Predictable updates** — atomic installs, safe rollback  
- **Cross‑platform** — Linux, Windows, macOS  
- **No feature gating** — CLI tools stay fully free  
- **Paid tier = convenience** — StaxDash adds dashboards, not restrictions  

This reboot is about building a **professional‑grade foundation** that can grow for years.

---

## 🛠 Current State

| Component | Status | Focus |
|----------|--------|--------|
| **StaxPing Classic** | `Legacy` | Original all‑in‑one tool (see `classic` branch) |
| **StaxPing 2.x** | `In Dev` | Clean rewrite with modular architecture |
| **StaxSpec** | `Planned` | System specs & resource monitoring |
| **StaxDash** | `Planned` | Unified GUI dashboard (paid convenience layer) |
| **Stax Registry** | `In Dev` | Central `stax.toml` for tool installs/updates |

---

## 🌐 Part of the Stax Ecosystem

StaxPing 2.x is no longer a standalone binary — it’s the **network module** in a larger sysadmin suite.

The ecosystem includes:

- a **shared installer/updater**  
- a **central registry (`stax.toml`)**  
- a **consistent folder structure**  
- **atomic updates**  
- **rollback support**  
- **tool discovery & installation**  

Example (planned):

```
staxping tools list
staxping tools install staxspec
staxping tools update --all
```

StaxPing becomes the **gateway tool** for the entire suite.

---

## 🚀 Installation (Coming Soon)

StaxPing 2.x will support:

- one‑command installation  
- automatic PATH setup  
- atomic updates  
- rollback to previous versions  
- multi‑tool installation via the Stax registry  

More details will be added as the new architecture lands.

---

## 📖 Documentation & Links

**Classic Version:**  
See the `classic` branch for the original StaxPing.

**Ecosystem Docs:**  
Will be published alongside the Stax registry and StaxSpec.

---

## 🧭 Roadmap (StaxPing 2.x)

### **Phase 1 — Foundation**
- New repo structure  
- New README  
- Stax registry (`stax.toml`)  
- Installer/updater design  
- Folder layout (`~/.stax/…`)  

### **Phase 2 — Updater**
- Download + install  
- Atomic swap  
- Rollback  
- Tool discovery  
- Tool installation  

### **Phase 3 — StaxPing Core**
- DNS  
- ICMP  
- HTTP  
- Traceroute  
- JSON output  

### **Phase 4 — Ecosystem Integration**
- StaxSpec support  
- StaxDash integration  
- Multi‑tool workflows  

---

## 👥 Credits

Created and maintained by **ZFordDev**.  
StaxPing 2.x is built with the lessons learned from the classic version and the vision for a clean, modular sysadmin toolkit.

---

## ❤️ Support the Project

If you want to support the development of the new Stax ecosystem:

- ⭐ **Star the repo**  
- ☕ **Ko‑Fi**: [https://ko-fi.com/zforddev](https://ko-fi.com/zforddev)  

---
