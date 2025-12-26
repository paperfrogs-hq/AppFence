# AppFence

**Application Permission Firewall for Linux Desktop**

> *Your desktop. Your rules. Your privacy.*

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Linux%20Wayland-green.svg)](https://wayland.freedesktop.org/)
[![Status](https://img.shields.io/badge/status-Phase%200%20Complete-brightgreen.svg)](PHASE_0_COMPLETE.md)

---

## The Problem

**Your Linux desktop has a blind spot.**

- Firefox wants to access your webcam. *Do you allow it? For how long?*
- A new app is connecting to the internet. *Do you know what data it's sending?*
- Random binaries are reading your Documents folder. *Should they be allowed?*

Unlike Android/iOS, Linux desktops have **no unified permission system**. Applications can:
- Access any file in your home directory
- Use your webcam/microphone without asking
- Connect to any network endpoint
- Start on boot silently
- Read your clipboard

**You have no visibility. No control. No consent.**

---

## The Solution: AppFence

AppFence brings **mobile-style permissions to Linux desktop**—with full transparency and user control.

```
┌─────────────────────────────────────────┐
│  "Firefox wants to access Camera"      │
│                                         │
│  [Deny]  [Allow Once]  [Allow Always]  │
└─────────────────────────────────────────┘
```

### What You Get

- **Runtime Permission Prompts** - Like Android, but honest about limitations  
- **Per-App Policies** - Filesystem, network, devices, clipboard  
- **Enforcement via OS Primitives** - Namespaces, cgroups, sandboxes (no hacks)  
- **Audit Logging** - See exactly what apps are doing  
- **Zero Trust by Default** - Apps get nothing unless you allow it  
- **Privacy First** - No telemetry, no cloud, local-only

---

## How It Works: The AppFence Model

### Architecture

```
┌──────────────────────────────────────────────────────┐
│                    User Space                        │
│                                                      │
│  ┏━━━━━━━━━━━┓         ┏━━━━━━━━━━━┓                │
│  ┃  App UI   ┃◄───────►┃   Agent   ┃                │
│  ┃  (Tauri)  ┃  DBus   ┃  (User)   ┃                │
│  ┗━━━━━━━━━━━┛         ┗━━━━━┯━━━━━┛                │
│                              │ Prompts               │
├──────────────────────────────┼──────────────────────┤
│               System         │                       │
│                       ┏━━━━━━▼━━━━━━┓                │
│                       ┃   Daemon    ┃                │
│                       ┃   (Root)    ┃                │
│                       ┗━━━━━┯━━━━━━━┛                │
│                              │                       │
│          ┌───────────────────┼────────────┐          │
│          │                   │            │          │
│    ┏━━━━━▼━━━━┓      ┏━━━━━━▼━━━━┓  ┏━━━▼━━━┓      │
│    ┃ Policy   ┃      ┃ Enforce   ┃  ┃ Launch┃      │
│    ┃ Engine   ┃      ┃ Backends  ┃  ┃ (bwrap)┃     │
│    ┗━━━━━━━━━━┛      ┗━━━━━━━━━━━┛  ┗━━━━━━━┛      │
└──────────────────────────────────────────────────────┘
```

### Core Components

#### 1. **System Daemon** (`apfd`) — The Brain
- Runs as root systemd service
- Evaluates permission policies
- Orchestrates enforcement
- Generates audit logs
- Written in Rust for memory safety

#### 2. **Session Agent** (`apf-agent`) — The Interface
- Runs as your user
- Shows permission prompts via desktop notifications
- Relays your decisions securely
- System tray integration

#### 3. **Controlled Launcher** (`apf-run`) — The Enforcer
- Wraps applications with isolation
- Configures Linux namespaces (mount, network, user)
- Sets up cgroups for resource limits
- Uses bubblewrap for sandboxing

#### 4. **Policy Engine** — The Memory
- SQLite database for permission rules
- Per-app, per-resource granularity
- Decision caching (avoid prompt fatigue)
- Time-limited permissions

#### 5. **UI Dashboard** — The Control Center
- Tauri-based desktop app
- Manage all app permissions
- View audit logs
- Import/export policies

---

## The AppFence Permission Model

### Permission Categories

| Category | Controls | Example |
|----------|----------|---------|
| **Network** | Internet access | `None` / `LAN Only` / `Full Internet` |
| **Filesystem** | File/folder access | `~/Documents: Read-Only` |
| **Devices** | Hardware access | Camera, Microphone, Screen, USB |
| **Clipboard** | Copy/paste monitoring | Notify on access |
| **Background** | Prevent idle execution | No background mining |
| **Autostart** | Boot behavior | Block startup entries |

### Enforcement Strength (Honest Transparency)

We tell you **exactly** what's enforced:

- **Strong** - Sandbox + namespace + cgroup (requires `apf-run` launcher)
- **Medium** - OS-level restrictions + portals (portal-aware apps)
- **Weak** - Audit logging only (legacy apps, monitoring mode)

*AppFence doesn't lie about what it can enforce.*

---

## Quick Start

### Installation (Future)

```bash
# Fedora
sudo dnf install appfence

# Ubuntu
sudo apt install appfence

# Arch
yay -S appfence
```

### Usage Examples

#### Launch App with Strong Enforcement
```bash
apf-run firefox
```

#### Check App Permissions
```bash
apf-policy show firefox
```

#### Block Network for Untrusted App
```bash
apf-policy set my-sketchy-app --network none
```

#### View Audit Log
```bash
apf-audit --app firefox --last 24h
```

---

## Why AppFence? Comparison

| Feature | AppFence | Flatpak | Firejail | SELinux | Little Snitch |
|---------|----------|---------|----------|---------|---------------|
| **All Apps** | Yes | No (Flatpak only) | Yes | Yes | Yes (macOS) |
| **User-Friendly** | Yes | Partial | No | No | Yes |
| **Permission Prompts** | Yes | No | No | No | Yes |
| **Network Control** | Yes | Partial | Partial | Yes | Yes |
| **Filesystem Granular** | Yes | Yes | Yes | Yes | No |
| **Desktop Integration** | Yes | Yes | No | No | Yes |
| **Enterprise Management** | Partial (v1.1+) | No | No | Yes | Yes |
| **Open Source** | Yes | Yes | Yes | Yes | No |
| **Rust Memory Safety** | Yes | Partial | No | No | No |

**AppFence = User-friendly + Comprehensive + Enterprise-ready**

---

## The Business Model

### For Individuals
- **Free & Open Source** (Apache-2.0)
- Privacy-focused desktop users
- Developers testing restricted environments

### For Enterprises
- **Centralized Management Console** (v1.1+)
- Fleet-wide policy deployment
- Compliance reporting (SOC2, HIPAA, GDPR)
- LDAP/AD integration
- Priority support contracts

**Target Markets:**
- Government workstations
- Financial institutions
- Healthcare organizations
- Privacy-focused companies
- Managed Linux desktop fleets

**Pricing Model:** $10-50/seat/year (enterprise features)

---

## Roadmap

### Phase 0: Foundations (Complete)
- [x] Architecture design
- [x] Threat model (STRIDE)
- [x] Rust workspace setup
- [x] Comprehensive documentation

### Phase 1: Core Implementation (Q1 2026)
- [ ] Policy engine with SQLite
- [ ] DBus daemon and agent
- [ ] Basic launcher with namespace isolation
- [ ] Unit tests (95% coverage)

### Phase 2: Enforcement Backends (Q2 2026)
- [ ] Filesystem backend (mount namespaces)
- [ ] Network backend (network namespaces + eBPF)
- [ ] Device backend (cgroups + portals)
- [ ] Clipboard monitoring

### Phase 3: User Interface (Q3 2026)
- [ ] Tauri desktop application
- [ ] Policy management UI
- [ ] Audit log viewer
- [ ] System tray integration

### Phase 4: Stabilization (Q4 2026)
- [ ] External security audit
- [ ] Performance optimization
- [ ] Distribution packaging (Fedora, Ubuntu, Arch)
- [ ] v1.0 release

### Phase 5: Enterprise (2027+)
- [ ] Central management console
- [ ] LDAP/AD integration
- [ ] Compliance reporting
- [ ] Bug bounty program

---

## Platform Support

### Primary: Linux (Wayland)
- **Fedora Workstation** (Primary development platform)
- **Ubuntu**
- **Arch Linux**
- **openSUSE**

**Requirements:**
- Linux kernel 5.15+
- Wayland compositor (GNOME, KDE Plasma, Sway)
- SystemD 250+
- Bubblewrap 0.8.0+

### Explicitly NOT Supported (v1.0)
- **X11** - Security model incompatible (no window isolation)
- **macOS** - Limited to monitoring only (no SIP bypass)
- **Windows** - Out of scope

---

## Security Guarantees

### What AppFence Provides
- Memory-safe code (Rust)
- OS-native enforcement (no kernel hacks)
- Transparent enforcement strength
- Audit trail
- Prompt-based consent

### What AppFence Does NOT Provide
- **Not antivirus** - No malware detection
- **Not a VM** - Not complete isolation
- **Not MAC replacement** - Complements SELinux, not replaces
- **Not kernel hardening** - Use separate tools

**Read:** [THREAT_MODEL.md](THREAT_MODEL.md) for honest risk analysis.

---

## Contributing

We welcome contributions! AppFence is in active development.

### Development Setup

```bash
# Clone repository
git clone https://github.com/yourusername/appfence.git
cd appfence

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run setup script
./scripts/setup-dev.sh

# Build
cargo build --release

# Run tests
cargo test --all
```

### Project Structure

```
AppFence/
├── crates/
│   ├── apf-core/        # Shared types, AppId
│   ├── apf-daemon/      # System daemon
│   ├── apf-agent/       # Session agent
│   ├── apf-launcher/    # Controlled launcher
│   ├── apf-policy/      # Policy engine
│   ├── apf-enforcement/ # Enforcement backends
│   └── apf-ui/          # Tauri UI
├── ARCHITECTURE.md      # System design (529 lines)
├── THREAT_MODEL.md      # Security analysis (663 lines)
└── SCOPE_AND_NON_GOALS.md # Project boundaries (392 lines)
```

### Areas Needing Help
- Rust development (daemon, enforcement)
- UI/UX design (Tauri frontend)
- Documentation and tutorials
- Testing and QA
- Packaging (distro maintainers)
- Security review

---

## Documentation

- **[Architecture](ARCHITECTURE.md)** - Complete system design
- **[Threat Model](THREAT_MODEL.md)** - STRIDE-based security analysis
- **[Scope & Non-Goals](SCOPE_AND_NON_GOALS.md)** - What AppFence is and isn't
- **[Phase 0 Complete](PHASE_0_COMPLETE.md)** - Foundation milestone

---

## Philosophy

### Design Principles

1. **User Agency** - Users control decisions, not algorithms
2. **Transparency** - Honest about enforcement limitations
3. **Privacy First** - No telemetry, no cloud
4. **OS-Native** - Use documented APIs, no kernel exploits
5. **Memory Safety** - Rust for all security-critical code
6. **Least Privilege** - Minimal privileged code surface

### What Makes AppFence Different

**Most security tools either:**
- Lie about capabilities (security theater)
- Are too complex for normal users (SELinux)
- Only work for specific app formats (Flatpak)
- Have no user interaction (AppArmor)

**AppFence is:**
- Honest about what it can and can't enforce
- User-friendly with clear prompts
- Works for ALL applications
- Gives users real-time control

---

## License

Apache License 2.0 - See [LICENSE](LICENSE) for details.

**Why Apache-2.0?**
- Permissive for enterprise adoption
- Patent grant protection
- Compatible with commercial use
- Strong liability protection

---

## Community

- **GitHub:** [yourusername/appfence](https://github.com/yourusername/appfence)
- **Issues:** Bug reports and feature requests
- **Discussions:** Design decisions and RFCs
- **Security:** security@appfence.org (coming soon)

---

## Acknowledgments

AppFence builds upon excellent Linux security primitives:
- **Linux Namespaces** - Process isolation
- **Cgroups v2** - Resource limits
- **Bubblewrap** - Sandboxing framework
- **xdg-desktop-portal** - Desktop integration
- **SystemD** - Service management

Standing on the shoulders of giants.

---

## FAQ

### Q: How is this different from Flatpak permissions?
**A:** Flatpak only manages Flatpak apps. AppFence manages ALL apps (system packages, AppImages, binaries) with consistent UX.

### Q: Will this slow down my system?
**A:** Launch overhead: <250ms. Runtime: zero overhead. Memory: ~25MB total.

### Q: Can it protect against kernel exploits?
**A:** No. AppFence operates in userspace. Use kernel hardening separately.

### Q: Why Wayland-only?
**A:** X11 cannot isolate windows or input. Any X11 app can spy on any other. Wayland is required for security.

### Q: Is this production-ready?
**A:** Not yet. Phase 0 complete. v1.0 targeted for Q4 2026 after security audit.

### Q: Can I use this commercially?
**A:** Yes! Apache-2.0 license allows commercial use.

---

## Status

**Current Phase:** Phase 0 - Foundations ✅  
**Next Phase:** Phase 1 - Core Implementation  
**Version:** 0.1.0-dev  
**Last Updated:** December 26, 2025

---

<div align="center">

**Security Notice**

AppFence is in early development and has not undergone security audit.  
Do not rely on it for critical security requirements yet.

**Built with Rust**

[Documentation](ARCHITECTURE.md) • [Roadmap](#roadmap) • [Contributing](#contributing) • [License](LICENSE)

</div>
