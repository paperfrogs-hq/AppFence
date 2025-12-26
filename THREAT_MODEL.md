# AppFence Threat Model

**Version:** 0.1.0  
**Last Updated:** December 26, 2025  
**Methodology:** STRIDE + Attack Trees

---

## 1. Introduction

This document analyzes security threats to AppFence and documents mitigations. We explicitly acknowledge residual risks and do not claim perfect security.

### Threat Modeling Scope
- **In Scope:** APF components, privilege boundaries, user interactions
- **Out of Scope:** Kernel vulnerabilities, hardware attacks, physical access
- **Trust Assumptions:** Kernel is trusted, systemd is trusted, DBus daemon is trusted

---

## 2. System Assets

### Critical Assets
1. **Privileged Daemon (`apfd`)** — Root access, policy enforcement
2. **Policy Database** — Permission decisions, audit logs
3. **User Decisions** — Consent for sensitive permissions
4. **Application Identity** — Basis for all policy decisions

### Asset Criticality
| Asset | Confidentiality | Integrity | Availability |
|-------|----------------|-----------|--------------|
| Daemon | Medium | **Critical** | High |
| Policy DB | Low | **Critical** | High |
| User Decisions | Low | **Critical** | Medium |
| AppId | Low | **Critical** | High |

---

## 3. Threat Actors

### A1: Malicious Application
- **Motivation:** Bypass restrictions, access protected resources
- **Capabilities:** User-level process, standard Linux APIs
- **Examples:** Spyware, adware, legitimate apps with malicious updates

### A2: Local Attacker (Non-Root)
- **Motivation:** Privilege escalation, data theft
- **Capabilities:** Shell access, multiple processes, system knowledge
- **Examples:** Compromised user account, insider threat

### A3: Social Engineering
- **Motivation:** Trick user into granting permissions
- **Capabilities:** UI manipulation, phishing techniques
- **Examples:** Fake permission dialogs, prompt fatigue exploitation

### A4: Compromised Dependencies
- **Motivation:** Supply chain attack
- **Capabilities:** Code execution in APF context
- **Examples:** Malicious crate, compromised build system

---

## 4. Threat Analysis (STRIDE)

### T1: Spoofing — Application Identity

#### T1.1: Desktop Entry Spoofing
**Description:** Malicious app creates `.desktop` file mimicking trusted app

**Attack Scenario:**
```
1. Attacker creates ~/.local/share/applications/firefox.desktop
2. Points to malicious binary
3. User launches via desktop menu
4. apf-run derives AppId from spoofed .desktop file
5. Malicious app inherits Firefox permissions
```

**Likelihood:** Medium  
**Impact:** High

**Mitigations:**
- AppId includes canonical executable path
- Binary hash verification (optional but recommended)
- Display full path in permission prompts
- Warn on identity mismatches

**Residual Risk:** Medium  
**Acceptance Rationale:** Users must verify application sources

---

#### T1.2: Executable Path Manipulation
**Description:** Symlink or rename attack to impersonate trusted binary

**Attack Scenario:**
```
1. Attacker creates ~/bin/gnome-terminal (malicious)
2. Modifies PATH to prioritize ~/bin
3. User runs "gnome-terminal"
4. apf-run resolves to malicious binary
```

**Likelihood:** Low  
**Impact:** High

**Mitigations:**
- Canonical path resolution (realpath)
- Hash-based change detection
- Warn on unusual installation paths

**Residual Risk:** Low  
**Acceptance Rationale:** Requires user cooperation or compromised environment

---

### T2: Tampering — Policy Modification

#### T2.1: Database Corruption
**Description:** Direct modification of SQLite policy database

**Attack Scenario:**
```
1. Attacker gains write access to /var/lib/appfence/policies.db
2. Modifies permissions for their malicious app
3. APF enforces attacker-controlled policy
```

**Likelihood:** Low  
**Impact:** Critical

**Mitigations:**
- Database file owned by root, mode 0600
- Daemon holds exclusive write lock
- Schema integrity checks on startup
- Audit log tamper detection (sequential IDs)

**Residual Risk:** Very Low  
**Acceptance Rationale:** Requires root compromise (game over anyway)

---

#### T2.2: DBus Message Injection
**Description:** Unauthorized DBus calls to modify policy

**Attack Scenario:**
```
1. Attacker crafts UpdatePolicy DBus message
2. Attempts to inject via org.appfence.Daemon interface
3. Policy for malicious app is granted full permissions
```

**Likelihood:** Low  
**Impact:** High

**Mitigations:**
- DBus policy file restricts method access
- Polkit authentication for privileged operations
- All updates require user authentication
- Rate limiting on policy changes

**Residual Risk:** Very Low  
**Acceptance Rationale:** Defense-in-depth via DBus + polkit

---

### T3: Repudiation — Audit Log Integrity

#### T3.1: Log Tampering
**Description:** Attacker deletes or modifies audit records

**Attack Scenario:**
```
1. Malicious app accesses sensitive resource
2. Attacker modifies audit.log to remove evidence
3. User unaware of unauthorized access
```

**Likelihood:** Low  
**Impact:** Medium

**Mitigations:**
- Logs stored in root-owned directory
- Write-only for daemon, no app access
- Integrity checksums (future: cryptographic signing)
- Off-system log forwarding (enterprise feature)

**Residual Risk:** Low  
**Acceptance Rationale:** Requires root access or focused forensic analysis

---

### T4: Information Disclosure — Data Leakage

#### T4.1: Audit Log Information Leakage
**Description:** Sensitive data exposed in logs

**Attack Scenario:**
```
1. Audit log records file paths: /home/user/Documents/passwords.txt
2. Attacker reads log file
3. Learns about sensitive file locations
```

**Likelihood:** Medium  
**Impact:** Low

**Mitigations:**
- Path redaction for home directory contents
- Log rotation and retention limits
- User-controlled log verbosity
- Logs stored locally only, no telemetry

**Residual Risk:** Low  
**Acceptance Rationale:** Local-only storage limits exposure

---

#### T4.2: Prompt Content Inference
**Description:** Side-channel attack via prompt timing

**Attack Scenario:**
```
1. Attacker monitors DBus traffic
2. Observes prompt requests for other apps
3. Infers user activity patterns
```

**Likelihood:** Low  
**Impact:** Very Low

**Mitigations:**
- DBus session bus is per-user
- Prompts don't expose file content, only permission types
- No cross-user information flow

**Residual Risk:** Very Low  
**Acceptance Rationale:** Minimal information gain

---

### T5: Denial of Service — Availability Attacks

#### T5.1: Prompt Flooding
**Description:** Malicious app triggers excessive prompts

**Attack Scenario:**
```
1. Malicious app requests 1000 permissions/second
2. User overwhelmed with prompts
3. System becomes unusable
```

**Likelihood:** High  
**Impact:** Medium

**Mitigations:**
- Rate limiting: Max 5 prompts/minute per app
- Auto-deny on rate limit exceeded
- Temporary app blocking (cooldown period)
- User notification of suspicious behavior

**Residual Risk:** Low  
**Acceptance Rationale:** Mitigation effectively limits impact

---

#### T5.2: Policy Database Lock Contention
**Description:** DoS via excessive policy queries

**Attack Scenario:**
```
1. Attacker spawns 1000 app instances simultaneously
2. All query policy database
3. SQLite write lock causes delays
```

**Likelihood:** Low  
**Impact:** Low

**Mitigations:**
- SQLite WAL mode (concurrent reads)
- In-memory policy cache with TTL
- Connection pooling in daemon
- Process spawn rate limiting via cgroups

**Residual Risk:** Very Low  
**Acceptance Rationale:** Performance optimization prevents issue

---

#### T5.3: Resource Exhaustion via Audit Logs
**Description:** Fill disk with excessive audit entries

**Attack Scenario:**
```
1. Malicious app performs 10M file accesses
2. Each logged to audit.log
3. Disk fills, system instability
```

**Likelihood:** Medium  
**Impact:** Medium

**Mitigations:**
- Log rotation (daily, max 7 days retained)
- Log rate limiting per app
- Disk space monitoring in daemon
- Graceful degradation (disable logging if disk full)

**Residual Risk:** Low  
**Acceptance Rationale:** Automated cleanup prevents accumulation

---

### T6: Elevation of Privilege

#### T6.1: Daemon Exploit
**Description:** Memory corruption in apfd leads to root compromise

**Attack Scenario:**
```
1. Attacker discovers buffer overflow in DBus handler
2. Sends crafted message to apfd
3. Gains root shell
```

**Likelihood:** Very Low  
**Impact:** Critical

**Mitigations:**
- **Rust memory safety** (no unsafe code in privileged paths)
- Minimal attack surface (small daemon codebase)
- No complex parsing in root context
- Fuzzing of DBus interface
- Capability dropping (only CAP_SYS_ADMIN retained)

**Residual Risk:** Very Low  
**Acceptance Rationale:** Rust eliminates entire vulnerability classes

---

#### T6.2: Confused Deputy Attack
**Description:** Trick APF into granting permissions to wrong app

**Attack Scenario:**
```
1. Attacker launches app A
2. App A spawns app B (malicious)
3. App B requests permission
4. APF grants based on A's identity
5. B inherits permissions
```

**Likelihood:** Medium  
**Impact:** High

**Mitigations:**
- Permissions bound to specific PID
- Parent-child process tracking
- AppId verification at enforcement time
- No permission inheritance across exec()

**Residual Risk:** Low  
**Acceptance Rationale:** Process isolation prevents cross-app sharing

---

#### T6.3: Time-of-Check-Time-of-Use (TOCTOU)
**Description:** Binary replaced after AppId verification

**Attack Scenario:**
```
1. apf-run verifies /usr/bin/legitimate-app (trusted)
2. Attacker replaces binary (race condition)
3. apf-run executes malicious binary with trusted permissions
```

**Likelihood:** Very Low  
**Impact:** High

**Mitigations:**
- File descriptor passing (not path-based)
- Binary hash verification before exec
- Open file, verify, then exec (atomic)
- Mount namespace prevents external modifications

**Residual Risk:** Very Low  
**Acceptance Rationale:** Multiple layers prevent race

---

### T7: Social Engineering

#### T7.1: Prompt Spoofing
**Description:** Fake permission dialog mimicking APF

**Attack Scenario:**
```
1. Malicious app displays window identical to APF prompt
2. User clicks "Allow"
3. App records decision, no actual APF interaction
4. User believes permission was denied
```

**Likelihood:** Low  
**Impact:** Medium

**Mitigations:**
- Prompts use desktop notification system (not app windows)
- Unique visual style (system theme)
- Security indicator in notification
- Apps cannot intercept notification clicks

**Residual Risk:** Very Low  
**Acceptance Rationale:** Desktop compositor prevents window spoofing

---

#### T7.2: Prompt Fatigue
**Description:** User blindly approves due to excessive prompts

**Attack Scenario:**
```
1. Legitimate apps request many permissions
2. User develops "click fatigue"
3. Malicious app requests sensitive permission
4. User approves without reading
```

**Likelihood:** **High**  
**Impact:** High

**Mitigations:**
- Sensitivity-aware prompting (batch low-risk permissions)
- Time-limited approvals (expire after 1 hour)
- Grouped permission requests
- Plain language, no jargon
- Visual risk indicators (color coding)
- Periodic permission review reminders

**Residual Risk:** **Medium**  
**Acceptance Rationale:** Human factor, cannot be fully mitigated

---

### T8: Enforcement Bypass

#### T8.1: Direct System Call Access
**Description:** App bypasses libc wrappers, calls kernel directly

**Attack Scenario:**
```
1. APF intercepts glibc file access functions
2. Malicious app uses raw syscall() for file access
3. Bypasses APF monitoring
```

**Likelihood:** High  
**Impact:** High

**Mitigations:**
- **Enforcement at kernel level** (namespaces, not library hooks)
- No LD_PRELOAD-based interception
- Seccomp filters (future enhancement)
- Honest disclosure: strong enforcement requires apf-run launcher

**Residual Risk:** **Medium**  
**Acceptance Rationale:** Apps outside apf-run are not protected (documented)

---

#### T8.2: Portal Avoidance
**Description:** App directly accesses device instead of using portal

**Attack Scenario:**
```
1. APF expects apps to use xdg-desktop-portal for camera
2. Malicious app opens /dev/video0 directly
3. Bypasses portal permission checks
```

**Likelihood:** High  
**Impact:** Medium

**Mitigations:**
- Device cgroup restrictions (strong enforcement)
- Udev rules for device access control
- Warn users about portal-aware vs. legacy apps
- Future: Mandatory portal usage via seccomp

**Residual Risk:** **Medium**  
**Acceptance Rationale:** Legacy app compatibility vs. security tradeoff

---

## 5. Attack Trees

### Attack Goal: Access User's Camera Without Permission

```
[Access Camera Without Permission]
├── OR: Bypass APF Enforcement
│   ├── AND: Launch Outside apf-run
│   │   ├── User launches app directly (HIGH LIKELIHOOD)
│   │   └── App accesses /dev/video0 (HIGH SUCCESS)
│   │   └── MITIGATION: Desktop environment integration
│   ├── AND: Exploit Daemon Vulnerability
│   │   ├── Find memory corruption bug (VERY LOW)
│   │   └── Gain root access (CRITICAL)
│   │   └── MITIGATION: Rust memory safety
│   └── AND: Use Portal Bypass
│       ├── Open device node directly (HIGH)
│       └── Evade device cgroup (MEDIUM)
│       └── MITIGATION: Device cgroup + udev rules
├── OR: Social Engineer User
│   ├── Display fake prompt (LOW)
│   │   └── MITIGATION: Desktop notifications
│   └── Induce prompt fatigue (HIGH)
│       └── MITIGATION: Grouped prompts, risk indicators
└── OR: Spoof Application Identity
    ├── Create fake .desktop entry (MEDIUM)
    │   └── MITIGATION: Binary hash verification
    └── Symlink to trusted app (LOW)
        └── MITIGATION: Canonical path resolution
```

---

## 6. Risk Matrix

| Threat ID | Likelihood | Impact | Risk Level | Mitigation Status |
|-----------|-----------|--------|------------|-------------------|
| T1.1 | Medium | High | **HIGH** | Partially Mitigated |
| T1.2 | Low | High | Medium | Mitigated |
| T2.1 | Low | Critical | Medium | Mitigated |
| T2.2 | Low | High | Medium | Mitigated |
| T3.1 | Low | Medium | Low | Mitigated |
| T4.1 | Medium | Low | Low | Mitigated |
| T4.2 | Low | Very Low | Very Low | Accepted |
| T5.1 | High | Medium | Medium | Mitigated |
| T5.2 | Low | Low | Very Low | Mitigated |
| T5.3 | Medium | Medium | Medium | Mitigated |
| T6.1 | Very Low | Critical | **LOW** | Mitigated (Rust) |
| T6.2 | Medium | High | Medium | Mitigated |
| T6.3 | Very Low | High | Low | Mitigated |
| T7.1 | Low | Medium | Low | Mitigated |
| T7.2 | **High** | High | **HIGH** | Partially Mitigated |
| T8.1 | High | High | **HIGH** | Partially Mitigated |
| T8.2 | High | Medium | **HIGH** | Partially Mitigated |

---

## 7. Residual Risks (Accepted)

### High-Risk Residuals

1. **Prompt Fatigue (T7.2)**
   - **Why Accept:** Fundamental human factor
   - **Ongoing Mitigation:** UX research, adaptive prompting

2. **Apps Outside apf-run (T8.1)**
   - **Why Accept:** User freedom vs. security tradeoff
   - **Disclosure:** Clearly documented limitation
   - **Long-term:** Desktop environment integration

3. **Identity Spoofing (T1.1)**
   - **Why Accept:** Requires user error or compromised .desktop
   - **Disclosure:** Security guidance, hash verification

---

## 8. Mitigations Roadmap

### Phase 1 (v0.1-0.5) — Core Security
- [x] Rust implementation (memory safety)
- [x] DBus access control
- [x] Polkit integration
- [x] Rate limiting
- [ ] Binary hash verification
- [ ] Audit log integrity checks

### Phase 2 (v0.6-1.0) — Enhanced Enforcement
- [ ] Seccomp filter integration
- [ ] eBPF-based monitoring
- [ ] Mandatory portal usage mode
- [ ] Desktop environment integration (Plasma/GNOME)

### Phase 3 (v1.1+) — Advanced Features
- [ ] Cryptographic log signing
- [ ] Hardware token integration
- [ ] ML-based anomaly detection
- [ ] Kernel LSM module (optional)

---

## 9. Security Assumptions

### We Assume:
1. **Kernel is trusted** — No kernel exploits considered
2. **SystemD is trusted** — Service isolation depends on it
3. **DBus daemon is trusted** — IPC security foundation
4. **User has physical access** — Not a remote attack scenario
5. **Bootloader is secure** — No Evil Maid attacks
6. **Hardware is not malicious** — No firmware backdoors

### We Do NOT Assume:
1. Applications are trustworthy
2. Users always make correct decisions
3. All apps use portals correctly
4. Desktop environments are bug-free
5. Third-party libraries are safe

---

## 10. Out-of-Scope Threats

### Explicitly Not Defended Against:
- Kernel vulnerabilities (use kernel hardening separately)
- Physical access attacks (use disk encryption)
- Supply chain attacks on system packages (use distro signatures)
- Side-channel attacks (Spectre, Meltdown, etc.)
- Nation-state adversaries with kernel exploits
- Malicious hardware (USB Rubber Ducky, etc.)

---

## 11. Compliance and Standards

### Alignment:
- **OWASP Top 10:** Addressed (injection, broken auth, etc.)
- **CWE Top 25:** Memory safety via Rust
- **STRIDE:** Full coverage
- **NIST Cybersecurity Framework:** Identify, Protect, Detect

---

## 12. Security Testing Plan

### Testing Strategy:
1. **Fuzzing:** DBus interface, policy parser
2. **Static Analysis:** Clippy, cargo-audit
3. **Dynamic Analysis:** Valgrind (for unsafe blocks)
4. **Penetration Testing:** Pre-release external audit
5. **Bug Bounty:** Post-1.0 public program

---

## 13. Incident Response

### Vulnerability Disclosure:
- **Email:** security@appfence.org
- **PGP Key:** [To be published]
- **Response SLA:** 48 hours acknowledgment
- **Fix Timeline:** 30 days for high severity

---

## 14. Conclusion

AppFence has a **realistic threat model** that acknowledges limitations while providing meaningful security improvements. Key strengths:

- Rust eliminates memory corruption
- OS-native enforcement is robust
- Transparency about limitations builds trust

Key weaknesses:
- Prompt fatigue remains a challenge
- Apps outside launcher reduce effectiveness
- Identity spoofing requires user awareness

**Overall Security Posture:** Suitable for privacy-conscious users and enterprise desktops with informed user base.

---

**Last Review:** December 26, 2025  
**Next Review:** Quarterly or on major architecture changes
