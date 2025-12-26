# AppFence Architecture

**Version:** 0.1.0  
**Target Platform:** Linux (Fedora Wayland primary)  
**Date:** December 26, 2025

---

## Executive Summary

AppFence is a Rust-based, policy-driven desktop permission management system that enables users to control application access to system resources through runtime prompts and OS-native enforcement mechanisms. It is **not** a sandbox, antivirus, or MAC replacement—it is a user-controlled permission orchestration layer.

---

## Design Principles

1. **OS-native enforcement only** — No kernel exploits, DRM bypasses, or undocumented hooks
2. **Rust for all security-critical code** — Memory safety, strict typing, explicit error handling
3. **Least privilege architecture** — Privileged code is minimal and isolated
4. **Transparency over absolute claims** — Enforcement strength is explicitly communicated
5. **Prompt-driven consent** — Users remain in control of sensitive decisions

---

## System Architecture

### High-Level Components

```
┌─────────────────────────────────────────────────────────────┐
│                        User Space                           │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐        ┌──────────────┐                  │
│  │  apf-ui      │◄──────►│  apf-agent   │                  │
│  │  (Tauri)     │  DBus  │  (User)      │                  │
│  └──────────────┘        └───────┬──────┘                  │
│                                   │ DBus                     │
├───────────────────────────────────┼──────────────────────────┤
│                        System     │                          │
├───────────────────────────────────┼──────────────────────────┤
│                           ┌───────▼──────┐                   │
│                           │  apfd        │                   │
│                           │  (Root)      │                   │
│                           └───────┬──────┘                   │
│                                   │                          │
│              ┌────────────────────┼────────────────┐         │
│              │                    │                │         │
│        ┌─────▼─────┐      ┌──────▼──────┐  ┌─────▼─────┐   │
│        │Policy     │      │Enforcement  │  │apf-run    │   │
│        │Engine     │      │Backends     │  │(Launcher) │   │
│        └───────────┘      └─────────────┘  └───────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## Component Details

### 1. System Daemon (`apfd`)

**Privilege Level:** Root  
**Language:** Rust  
**Communication:** DBus system bus

**Responsibilities:**
- Policy storage and evaluation
- Enforcement orchestration
- Prompt request generation
- Audit logging
- AppId verification

**Security Boundaries:**
- Runs as systemd service with minimal capabilities
- No UI rendering or complex parsing
- Validates all DBus requests
- Rate-limits prompt requests

**DBus Interface:**
```
org.appfence.Daemon
├── RequestPermission(AppId, PermissionType) -> PromptId
├── ReportDecision(PromptId, Decision) -> bool
├── GetPolicy(AppId) -> PolicyDocument
└── UpdatePolicy(AppId, PolicyDocument) -> bool
```

---

### 2. Session Agent (`apf-agent`)

**Privilege Level:** User  
**Language:** Rust  
**Communication:** DBus session bus

**Responsibilities:**
- Display permission prompts via desktop notifications
- Relay user decisions to daemon
- System tray integration
- Desktop portal integration

**Security Boundaries:**
- Cannot make policy decisions directly
- All decisions validated by daemon
- Prompt IDs bind to specific requests

**DBus Interface:**
```
org.appfence.Agent
├── ShowPrompt(PromptId, AppId, PermissionType) -> void
└── NotifyEnforcement(AppId, Result) -> void
```

---

### 3. Controlled Launcher (`apf-run`)

**Privilege Level:** User  
**Language:** Rust

**Responsibilities:**
- Derive application identity (AppId)
- Query policy from daemon
- Setup sandbox (bubblewrap)
- Configure namespaces (user, network, mount)
- Configure cgroups v2
- Execute target application

**Enforcement Stack:**
```
Application Process
  └─> Cgroup (resource limits)
      └─> User Namespace (UID mapping)
          └─> Network Namespace (optional isolation)
              └─> Mount Namespace (filesystem restrictions)
                  └─> Bubblewrap (additional sandboxing)
```

**AppId Derivation Priority:**
1. Flatpak ID (if launched via Flatpak)
2. XDG Desktop Entry ID
3. Canonical executable path
4. Optional: SHA-256 binary hash

---

### 4. Policy Engine (`apf-policy`)

**Storage:** SQLite database  
**Location:** `/var/lib/appfence/policies.db`

**Schema:**
```sql
CREATE TABLE policies (
    app_id TEXT PRIMARY KEY,
    policy_json TEXT NOT NULL,
    created_at INTEGER,
    updated_at INTEGER
);

CREATE TABLE decisions (
    id INTEGER PRIMARY KEY,
    app_id TEXT,
    permission_type TEXT,
    decision TEXT,
    expires_at INTEGER,
    created_at INTEGER
);

CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY,
    timestamp INTEGER,
    app_id TEXT,
    permission_type TEXT,
    action TEXT,
    result TEXT
);
```

**Policy Document Format:**
```json
{
  "app_id": "org.example.app",
  "version": 1,
  "permissions": {
    "network": "internet",
    "filesystem": [
      {"path": "/home/user/Documents", "mode": "read-write"},
      {"path": "/etc", "mode": "read-only"}
    ],
    "devices": ["camera"],
    "clipboard": true,
    "background": false,
    "autostart": false
  },
  "enforcement": "strong"
}
```

---

### 5. Enforcement Backends (`apf-enforcement`)

#### Filesystem Backend
- **Strong:** Mount namespace + bind mounts
- **Medium:** File descriptor passing via portals
- **Weak:** Audit only

#### Network Backend
- **Strong:** Network namespace + eBPF filters
- **Medium:** Firewall rules (nftables/iptables)
- **Weak:** Connection monitoring

#### Device Backend
- **Strong:** Device cgroup + portal integration
- **Medium:** udev rules + polkit
- **Weak:** Access logging

#### Clipboard Backend
- **Medium:** Wayland clipboard portal
- **Weak:** Notification on access

---

### 6. User Interface (`apf-ui`)

**Technology:** Tauri (Rust + Web)  
**Privilege:** User

**Features:**
- Policy management dashboard
- Application permission matrix
- Audit log viewer
- Quick toggles for common apps
- Search and filter
- Export/import policies

---

## Communication Patterns

### Prompt Flow
```
1. Application → apf-run → Detects permission needed
2. apf-run → apfd (DBus) → RequestPermission
3. apfd → Evaluates policy, decides prompt needed
4. apfd → apf-agent (DBus) → ShowPrompt
5. apf-agent → Desktop notification shown
6. User → Clicks decision
7. apf-agent → apfd → ReportDecision
8. apfd → Validates, stores decision
9. apfd → apf-run → Returns result
10. apf-run → Applies enforcement
```

### Security Properties
- Prompts cannot be triggered by apps directly
- Prompt IDs are unique and expire
- Decisions are bound to AppId + PID
- Rate limiting prevents prompt spam

---

## Technology Stack

### Core
- **Rust:** 1.75+ (stable)
- **tokio:** Async runtime
- **zbus:** DBus protocol
- **serde:** Serialization
- **rusqlite:** Policy storage
- **tracing:** Structured logging

### System Integration
- **systemd:** Service management
- **polkit:** Privileged operations
- **bubblewrap:** Sandboxing
- **cgroups v2:** Resource control
- **xdg-desktop-portal:** Wayland integration

### UI
- **Tauri 2.0:** Desktop app framework

---

## Platform Support

### Primary: Fedora Wayland
- Full feature support
- Strong enforcement available
- Desktop portal integration
- SystemD native

### Secondary: Ubuntu, Arch, openSUSE
- Same capabilities as Fedora
- Minor packaging differences

### Explicitly Not Supported (v1.0):
- **X11:** Security model incompatible
- **macOS:** Limited to monitoring only
- **Windows:** Out of scope

---

## Security Architecture

### Privilege Separation
```
Root Domain:
  - apfd daemon (minimal capabilities)
  - Policy database (read/write)
  
User Domain:
  - apf-agent (no privileged access)
  - apf-ui (read-only policy view)
  - apf-run (unprivileged sandbox setup)
```

### Trust Boundaries
1. **Daemon ↔ Agent:** Authenticated via DBus
2. **Daemon ↔ Launcher:** Validated AppId
3. **Agent ↔ User:** Desktop notification system
4. **Launcher ↔ App:** Kernel namespaces

### Attack Surface Minimization
- Daemon has no network access
- No dynamic code loading
- Minimal parsing in privileged code
- All user input validated before root context

---

## Deployment Model

### System Installation
```
/usr/bin/
  ├── apfd
  ├── apf-agent
  ├── apf-run
  └── apf-ui

/usr/lib/systemd/system/
  └── appfence.service

/usr/lib/systemd/user/
  └── appfence-agent.service

/etc/appfence/
  └── default-policies.d/

/var/lib/appfence/
  ├── policies.db
  └── audit.log

/usr/share/dbus-1/system.d/
  └── org.appfence.Daemon.conf

/usr/share/dbus-1/services/
  └── org.appfence.Agent.service
```

---

## Performance Considerations

### Launch Overhead
- **Baseline:** ~50ms (AppId derivation + policy lookup)
- **Sandbox setup:** +100-200ms
- **Acceptable:** <250ms total for strong enforcement

### Memory Footprint
- **Daemon:** ~10MB RSS
- **Agent:** ~15MB RSS
- **Per-app overhead:** ~5MB (namespace metadata)

### Database Performance
- SQLite with WAL mode
- Indexed on app_id
- Expected: <1ms policy lookups
- Audit logs rotated daily

---

## Future Considerations

### Phase 2 Enhancements
- eBPF-based network filtering
- Kernel LSM module (optional)
- Hardware token integration
- Remote policy management (enterprise)

### Phase 3 Enterprise
- Fleet management console
- LDAP/AD integration
- Compliance reporting
- Centralized audit aggregation

---

## Limitations and Non-Goals

### Explicitly NOT Provided
- Malware detection
- Antivirus functionality
- Complete application isolation (not a VM)
- Mandatory Access Control replacement
- Protection against kernel exploits
- Time-travel debugging

### Known Limitations
- Apps launched outside `apf-run` bypass controls
- X11 apps cannot be strongly isolated
- Requires Wayland for full feature set
- Root processes cannot be constrained

---

## References

- [systemd Resource Control](https://www.freedesktop.org/software/systemd/man/systemd.resource-control.html)
- [Bubblewrap Documentation](https://github.com/containers/bubblewrap)
- [xdg-desktop-portal](https://flatpak.github.io/xdg-desktop-portal/)
- [Linux Namespaces](https://man7.org/linux/man-pages/man7/namespaces.7.html)

---

**Document Status:** Living document, updated with implementation progress
