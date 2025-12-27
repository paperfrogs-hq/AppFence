# AppFence

**Application Permission Firewall for Linux Desktop**

> *Your desktop. Your rules. Your privacy.*

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Linux%20Wayland-green.svg)](https://wayland.freedesktop.org/)

---

## Security Notice

AppFence is in early development and has **not** undergone a security audit. Do not rely on it for critical security requirements yet.

---

## The Problem

Linux desktops lack a unified permission model. Applications can access your files, camera/microphone, network, and clipboard without meaningful consent.

---

## What AppFence Does

- Runtime prompts for filesystem, network, devices, and clipboard access
- Per-application policies with allow/deny/ask controls
- Enforcement through Linux namespaces, cgroups, and sandboxing
- Local-only auditing and visibility

---

## Architecture at a Glance

- **Daemon (`apfd`)**: system service that evaluates policies and coordinates enforcement
- **Agent (`apf-agent`)**: user-session component that surfaces prompts and collects decisions
- **Launcher (`apf-run`)**: wraps application startup to apply namespaces/sandboxing
- **Policy Engine (`apf-policy`)**: stores and evaluates per-app rules
- **UI (`apf-ui`)**: desktop dashboard for managing permissions and reviewing audits

---

## Quick Start (preview)

```bash
# build workspace
cargo build --workspace

# run setup helpers
./scripts/setup-dev.sh

# launch an app with enforcement
apf-run firefox
```

---

## Comparison (high level)

| Feature | AppFence | Flatpak | Firejail |
|---------|----------|---------|----------|
| Works for all apps | Yes | No (packaged apps) | Yes |
| Permission prompts | Yes | No | No |
| Network control | Yes | Partial | Partial |
| Filesystem granularity | Yes | Yes | Yes |
| Desktop integration | Yes | Partial | Limited |

---

## Philosophy

- User agency first: explicit prompts and clear defaults
- Transparent limits: state what can and cannot be enforced
- Privacy by default: no telemetry, local policy storage
- OS-native: rely on documented Linux primitives, avoid kernel hacks
- Memory safety: Rust for security-sensitive components

---

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
