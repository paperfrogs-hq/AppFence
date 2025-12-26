# AppFence: Scope and Non-Goals

**Version:** 0.1.0  
**Date:** December 26, 2025  
**Purpose:** Define what AppFence IS and IS NOT to prevent scope creep and manage expectations

---

## Executive Summary

AppFence is a **user-controlled permission orchestration layer** for Linux desktop applications. It is designed to give users visibility and control over application behavior through runtime prompts and OS-native enforcement mechanisms.

**What it IS:** A permission firewall  
**What it is NOT:** A sandbox, antivirus, or security silver bullet

---

## 1. Project Scope (WHAT WE BUILD)

### 1.1 Core Functionality ✅

#### Permission Management
- **Network access control** (none / LAN / internet)
- **Filesystem access** (per-directory read/write/deny)
- **Device access** (camera, microphone, screen, USB)
- **Clipboard access** monitoring and control
- **Background execution** prevention
- **Autostart** management

#### User Interface
- **Runtime permission prompts** via desktop notifications
- **Policy management dashboard** (Tauri-based GUI)
- **Audit log viewer** showing application behavior
- **Quick toggles** for common applications
- **Import/export** policies

#### Enforcement
- **Controlled application launcher** (`apf-run`)
- **Sandbox integration** (bubblewrap)
- **Linux namespace** isolation (user, network, mount)
- **Cgroup resource** limits
- **Desktop portal** integration (Wayland)
- **Enforcement strength** transparency (strong/medium/weak)

#### System Integration
- **SystemD services** (daemon + user agent)
- **DBus interfaces** for IPC
- **Polkit** for privileged operations
- **XDG standards** compliance

---

### 1.2 Target Platforms ✅

#### Primary Support (Full Features)
- **Fedora Workstation** (Wayland)
- **Ubuntu** (Wayland session)
- **Arch Linux** (Wayland)
- **openSUSE Tumbleweed** (Wayland)

#### Minimum Requirements
- **Linux kernel** 5.15+ (cgroups v2, modern namespaces)
- **Wayland compositor** (GNOME, KDE Plasma, Sway)
- **SystemD** 250+
- **Bubblewrap** 0.8.0+
- **xdg-desktop-portal** 1.18+

#### Explicitly Wayland-Only (v1.0)
- X11 support is **intentionally excluded** due to security model incompatibilities
- X11 clients on Xwayland receive limited enforcement (portal-level only)

---

### 1.3 User Personas ✅

#### Primary: Privacy-Conscious Desktop Users
- Wants control over application behavior
- Comfortable with technical prompts
- Values transparency over convenience

#### Secondary: System Administrators
- Manages multiple Linux workstations
- Needs fleet-wide policy deployment (Phase 2)
- Requires compliance reporting

#### Tertiary: Developers
- Tests applications under restricted environments
- Debugs permission issues
- Creates custom policies

---

### 1.4 Development Phases

#### Phase 0: Foundations ✅ (This Document)
- Project scope definition
- Architecture documentation
- Threat modeling
- Rust workspace setup

#### Phase 1: Core Implementation (v0.1-0.3)
- System daemon (apfd)
- Session agent (apf-agent)
- Controlled launcher (apf-run)
- Basic policy engine
- SQLite storage

#### Phase 2: Enforcement Backends (v0.4-0.6)
- Filesystem backend
- Network backend
- Device backend
- Clipboard monitoring
- Desktop portal integration

#### Phase 3: User Interface (v0.7-0.9)
- Tauri application
- Policy management UI
- Audit log viewer
- System tray integration

#### Phase 4: Stabilization (v1.0)
- Security audit
- Performance optimization
- Documentation
- Distribution packaging

#### Phase 5: Enterprise Features (v1.1+)
- Central management console
- LDAP/AD integration
- Fleet policy deployment
- Compliance reporting

---

## 2. Non-Goals (WHAT WE DON'T BUILD)

### 2.1 Security Non-Goals ❌

#### Not a Sandbox
- **We are NOT:** A general-purpose application sandbox
- **Reason:** Sandboxing is a complex, solved problem (Flatpak, Snap, Firejail)
- **Our role:** Orchestrate existing sandboxing tools, not replace them

#### Not Antivirus/Anti-Malware
- **We are NOT:** Malware detection or prevention system
- **Reason:** Requires signature databases, heuristics, constant updates
- **Our role:** Restrict malicious behavior, not identify malware

#### Not a Firewall
- **We are NOT:** Network firewall replacement
- **Reason:** iptables/nftables already exist
- **Our role:** Application-level network permission management

#### Not Mandatory Access Control (MAC)
- **We are NOT:** SELinux or AppArmor replacement
- **Reason:** Kernel-level MAC is a different security layer
- **Our role:** User-space permission orchestration

#### Not a Virtual Machine
- **We are NOT:** Complete isolation like QEMU/KVM
- **Reason:** Performance overhead, complexity
- **Our role:** Process-level isolation via namespaces

#### Not Kernel Hardening
- **We are NOT:** Grsecurity, PaX, or kernel hardening
- **Reason:** Kernel security is separate concern
- **Our role:** Userspace policy enforcement

---

### 2.2 Platform Non-Goals ❌

#### Not Supporting X11 (v1.0)
- **Reason:** X11 security model fundamentally incompatible
  - No window isolation
  - Global keyboard/mouse access
  - Clipboard is insecure by design
- **Decision:** Wayland-only for v1.0
- **Future:** X11 apps on Xwayland get limited support (portal-only)

#### Not Supporting macOS (Beyond Monitoring)
- **Reason:** 
  - No SIP bypass allowed
  - No kernel extension approval
  - Limited enforcement capabilities
- **Decision:** macOS version is policy management only (no enforcement)

#### Not Supporting Windows
- **Reason:** Out of scope, different security model
- **Decision:** Linux-first forever

#### Not Supporting Mobile (Android/iOS)
- **Reason:** Mobile platforms have permission systems
- **Decision:** Desktop Linux only

---

### 2.3 Feature Non-Goals ❌

#### Not a Browser Extension
- **We are NOT:** Browser-based permission manager
- **Reason:** Browsers have their own permission systems
- **Our role:** OS-level control only

#### Not Application Whitelisting
- **We are NOT:** Execution control system (only approved apps run)
- **Reason:** Breaks user freedom, maintenance burden
- **Our role:** Permission control for any app

#### Not Centralized Telemetry
- **We are NOT:** Collecting usage data or analytics
- **Reason:** Privacy-first design
- **Our role:** Local-only operation

#### Not Cloud Sync
- **We are NOT:** Syncing policies to cloud (v1.0)
- **Reason:** Privacy concerns
- **Future:** Optional enterprise feature with on-premise control

#### Not App Store / Repository
- **We are NOT:** Managing application installation
- **Reason:** Distros already handle this
- **Our role:** Permission management post-installation

#### Not a Learning System (v1.0)
- **We are NOT:** ML-based behavior analysis (initially)
- **Reason:** Complexity, false positives
- **Future:** Phase 3 consideration for anomaly detection

---

### 2.4 User Experience Non-Goals ❌

#### Not Invisible/Automatic
- **We are NOT:** Making decisions without user consent
- **Reason:** User agency is core principle
- **Our role:** Prompt-driven, user-controlled

#### Not Zero-Configuration
- **We are NOT:** "Set and forget" solution
- **Reason:** Meaningful security requires user engagement
- **Our role:** Provide good defaults, require confirmation

#### Not Enterprise Single Sign-On (v1.0)
- **We are NOT:** Integrating with corporate SSO initially
- **Reason:** Adds complexity
- **Future:** Phase 5 enterprise feature

---

## 3. Boundaries and Constraints

### 3.1 Technical Boundaries

#### Kernel Trust Boundary
- **We trust:** Linux kernel, SystemD, DBus daemon
- **We don't trust:** Applications, user decisions, third-party libraries

#### Enforcement Limitations
```
Strong Enforcement (Sandbox + Namespace):
  ✅ Apps launched via apf-run
  ❌ Apps launched outside apf-run

Medium Enforcement (OS restrictions):
  ✅ Portal-aware applications
  ❌ Legacy apps bypassing portals

Weak Enforcement (Audit only):
  ✅ All apps (monitoring)
  ❌ No actual blocking
```

#### Performance Constraints
- **Launch overhead:** <250ms acceptable
- **Memory footprint:** <50MB total (daemon + agent)
- **Database queries:** <1ms for policy lookups
- **No background CPU usage** when idle

---

### 3.2 Legal Boundaries

#### We Do NOT:
- Modify application binaries
- Bypass DRM or encryption
- Reverse engineer proprietary software
- Violate DMCA anti-circumvention provisions
- Hook into kernel without permission

#### We DO:
- Use documented Linux APIs
- Respect application licenses
- Operate in userspace (with optional root daemon)
- Provide source code (Apache-2.0)
- Document all enforcement mechanisms

---

### 3.3 Organizational Boundaries

#### Open Source Model
- **Governance:** Community-driven (initially solo developer)
- **License:** Apache-2.0 (permissive, enterprise-friendly)
- **Development:** Public GitHub repository
- **Funding:** No venture capital (v1.0)

#### Not Affiliated With:
- Red Hat / Fedora (independent project)
- Canonical / Ubuntu
- KDE / GNOME foundations
- Linux Foundation (yet)

---

## 4. Success Criteria (How We Measure)

### 4.1 Technical Success Metrics

#### v1.0 Release Criteria
- [ ] 95% test coverage (core modules)
- [ ] Zero critical security vulnerabilities
- [ ] External security audit passed
- [ ] Package available in Fedora/Ubuntu repos
- [ ] <100ms policy lookup latency
- [ ] <5% user-reported prompt fatigue

#### Adoption Metrics (Year 1)
- [ ] 10,000 active installs
- [ ] 100 GitHub stars
- [ ] 5 enterprise pilot customers
- [ ] Featured in Linux publication (LWN, Phoronix)

---

### 4.2 User Success Metrics

#### Users Should Be Able To:
- Install APF in <5 minutes
- Understand first prompt without documentation
- Create custom policy in <2 minutes
- Identify unauthorized app behavior via audit log
- Recover from misconfiguration without terminal

---

### 4.3 Security Success Metrics

#### Threat Mitigation
- [ ] No memory-safety vulnerabilities (Rust guarantees)
- [ ] No privilege escalation vulnerabilities
- [ ] Prompt spoofing demonstrably prevented
- [ ] Identity spoofing detectable by users

---

## 5. Out-of-Scope Comparisons

### What AppFence Is NOT Competing With:

| Tool | Why Not Competing |
|------|-------------------|
| **Flatpak** | Complements it (manages Flatpak app permissions) |
| **SELinux** | Different layer (kernel MAC vs. userspace orchestration) |
| **Firejail** | Different focus (general sandbox vs. permission manager) |
| **Little Snitch** | Different platform (macOS vs. Linux) |
| **Windows Defender** | Different OS, different threat model |
| **Portmaster** | Similar but network-focused; we're broader |

---

## 6. Future Expansion Criteria

### When to Add New Features

#### Must Meet ALL Criteria:
1. **Aligns with core mission** (user-controlled permissions)
2. **User demand validated** (GitHub issues, surveys)
3. **No architectural rework** required
4. **Maintains performance** (<10% overhead)
5. **Security audit feasible** (no new attack surface)
6. **Open source ecosystem** benefits

#### Examples of Future Expansions:
- ✅ eBPF-based enforcement (natural evolution)
- ✅ LSM module (optional, kernel-level)
- ✅ Enterprise management (user demand)
- ❌ Windows port (out of scope)
- ❌ AI-based permission prediction (complexity)

---

## 7. Stakeholder Communication

### For Users:
> "AppFence gives you control over what apps can access on your Linux desktop. Think of it like Android permissions but for desktop Linux."

### For Developers:
> "AppFence is a Rust-based permission orchestration layer that integrates with existing Linux security primitives (namespaces, cgroups, portals) to provide user-friendly permission management."

### For Security Researchers:
> "AppFence is NOT a sandbox or MAC system. It's a user-space policy orchestrator that enhances visibility and control while acknowledging enforcement limitations. See THREAT_MODEL.md for details."

### For Enterprises:
> "AppFence provides desktop-level zero-trust controls with centralized management (v1.1+). It complements existing endpoint security solutions by adding granular application permission management."

---

## 8. Decision Log

### Key Architectural Decisions

| Decision | Rationale | Date |
|----------|-----------|------|
| Wayland-only (v1.0) | X11 security model incompatible | Dec 2025 |
| Rust implementation | Memory safety critical | Dec 2025 |
| Apache-2.0 license | Enterprise adoption | Dec 2025 |
| No telemetry | Privacy-first principle | Dec 2025 |
| Prompt-driven | User agency over automation | Dec 2025 |
| DBus IPC | Linux desktop standard | Dec 2025 |

---

## 9. Frequently Asked Questions

### Q: Why not just use Flatpak permissions?
**A:** Flatpak is great for Flatpak apps. APF manages ALL apps (system packages, AppImages, binaries) with consistent UX.

### Q: Can APF protect against kernel exploits?
**A:** No. If the kernel is compromised, game over. Use kernel hardening separately.

### Q: Will this break my games?
**A:** Strong enforcement may. We provide enforcement strength selection and per-app overrides.

### Q: Why not support X11?
**A:** X11 fundamentally cannot isolate windows or input. Any X11 app can spy on any other. Wayland is required for security.

### Q: Is this like Little Snitch for Linux?
**A:** Similar concept, broader scope. Little Snitch is network-focused; APF covers filesystem, devices, etc.

### Q: Can enterprises deploy this?
**A:** Yes (v1.1+). Phase 5 adds central management, policy deployment, and compliance reporting.

---

## 10. Review and Updates

### Living Document
This scope definition will be reviewed:
- **Quarterly** during active development
- **On major feature requests**
- **After security audits**
- **Before version milestones** (0.5, 1.0, 2.0)

### Change Process
1. Propose scope change via GitHub issue
2. Discuss with community (minimum 2 weeks)
3. Vote (maintainer decision initially, later governance model)
4. Update this document with rationale

---

## 11. Conclusion

AppFence is deliberately scoped to solve **one problem well**: giving Linux desktop users visibility and control over application permissions through a user-friendly, secure, and transparent system.

By clearly defining what we **are NOT building**, we:
- Prevent scope creep
- Set realistic expectations
- Focus development efforts
- Maintain security promises
- Enable future expansion

**Scope Status:** LOCKED for v1.0  
**Next Review:** March 2026 or after security audit

---

**Approved By:** Project Lead  
**Date:** December 26, 2025  
**Version:** 0.1.0
