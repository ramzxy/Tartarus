## Tartarus Technical Spec (Draft)

### Overview

Tartarus is a sandbox creation and management tool for observing the behavior of malware on a controlled, disposable virtual machine. A sample is uploaded through a dashboard, detonated inside a freshly provisioned VM, and its low-level activity is streamed back to the host in real time for display and analysis.

### Goals

The primary goal is a locally-hosted equivalent of any.run, tailored to malware behavior observation. The MVP target is:

- Launch a Linux sandbox VM on demand
- Accept a malware sample via the dashboard and execute it inside the VM
- Stream kernel-level telemetry from the guest back to the host
- Display that telemetry live in a web dashboard

Windows guest support is planned but explicitly out of scope for v1.

The author's primary purpose for this project at this stage is to learn. This is the author's first significant Rust project and a deliberate vehicle for learning Rust and systems programming. Decisions in this spec sometimes favor learning value over shipping speed, and that tradeoff is intentional.

### Non-Goals

Tartarus is **not**:

- A static analysis tool — there is no disassembly, decompilation, or signature matching
- A YARA or rule-based detection engine
- A multi-tenant or production-SOC-ready service
- An automated IOC extractor in v1 (raw telemetry only; structured IOC extraction is future work)
- A Windows malware analysis platform in v1
- A replacement for VirusTotal or other cloud reputation services

### Threat Model

Tartarus assumes the malware is the adversary and the host must remain trustworthy regardless of what runs in the guest. Threats considered:

- **Sandbox detection.** Malware may fingerprint the environment (QEMU device strings, CPU flags, low resource counts, lack of user activity) and refuse to detonate or alter its behavior. v1 accepts this risk; bypasses are incremental future work.
- **In-guest telemetry tampering.** Malware with kernel-level access in the guest can locate, kill, or feed false data to the agent. v1 accepts this risk and treats in-guest telemetry as a *hypothesis* rather than ground truth. Stealth-grade observation via host-side VMI is planned as future work to address this.
- **VM escape.** A successful escape from guest to host would compromise the entire system. v1 relies on stock KVM/QEMU isolation; no additional hardening beyond keeping the host kernel and QEMU patched.
- **Covert network exfiltration.** Malware may use protocols or channels the agent cannot observe. v1 mitigates this by routing all guest traffic through an isolated host-controlled bridge where host-side packet capture can be added (future work).

### Architecture

- **Orchestrator (host):** Creates, manages, and destroys sandbox VMs. Handles VM lifecycle, sample upload into the guest, and detonation triggering.
- **Guest Agent (in-VM):** Loads eBPF programs, drains the ring buffer, formats events as structured messages, and ships them to the host. Acts as the bridge between the in-guest kernel telemetry and the host-side bus.
- **Telemetry Bus (host):** Receives events from the guest agent and decouples telemetry production from consumption. Initial implementation: Redis queue. Consumers (dashboard, future analyzers) subscribe independently.
- **Dashboard (host):** Web UI that consumes events from the bus and renders them live. Hosted on the host machine, accessed via browser. Built on Python + Flask for v1.
- **VMI Layer (future):** Host-side memory introspection via LibVMI / DRAKVUF, providing stealth-grade telemetry as a complement to the in-guest agent. Out of scope for v1.

### Detonation Lifecycle

1. User uploads a sample, optionally with a custom detonation command, via the dashboard.
2. Orchestrator allocates a VM by cloning a golden snapshot.
3. Orchestrator boots the VM and waits for an agent heartbeat over vsock.
4. Orchestrator transfers the sample into the VM. *(Mechanism TBD — see Open Questions.)*
5. Orchestrator triggers the detonation script inside the VM.
6. Agent streams events back over vsock; the host pushes them onto the telemetry bus.
7. Dashboard renders events live as they arrive.
8. Run terminates on one of: timeout, user stop, agent crash, or VM crash.
9. Orchestrator collects final artifacts (memory dump, disk diff, network capture — exact set TBD).
10. Orchestrator destroys the VM and restores the snapshot.

### Success Criteria for v1

v1 is considered complete when all of the following are demonstrable:

- A known benign Linux ELF test sample can be detonated end-to-end via the dashboard.
- The agent produces a structured event stream with at least three event categories: process execution, file access, and network activity.
- A full run (boot → detonate → collect → teardown) completes in under 5min.
- The VM is fully destroyed and the snapshot restored after every run, with no leftover state on the host.
- The dashboard displays events live during detonation, not just after completion.

### Open Questions

Decisions to be made before or during implementation:

- **Sample transfer mechanism into the guest:** vsock, virtio-9p shared folder, or virtio-fs?
- **Event serialization format:** JSON (simple, debuggable), msgpack (compact, fast), or protobuf (schema'd, fastest)?
- **Telemetry bus choice:** Redis queue, NATS, direct websocket from agent to dashboard, or hybrid?
- **Custom guest kernel:** include kernel patches for agent hiding in v1, or punt to v2?
- **Default detonation timeout:** what's the v1 default, and is there a hard maximum?
- **Final artifact collection:** memory dump, disk diff, full pcap — which are in v1, which are future?
- **Sample isolation between runs:** how do we guarantee no cross-contamination between consecutive detonations on the same host?

### Future Work

Ideas explicitly out of scope for v1, captured here so they aren't lost:

- **Windows guest support.** Requires Sysmon/ETW-based agent instead of eBPF, plus Windows-specific golden image management.
- **VMI-based stealth telemetry.** Host-side observation via LibVMI/DRAKVUF, providing ground-truth data the malware cannot tamper with. Cross-correlation against in-guest agent output for tamper detection.
- **Custom guest kernel** with patches to hide the agent, eBPF surface, and kernel build identifiers from in-guest enumeration.
- **Host-side eBPF** on the tap interface (XDP/tc-bpf) for ground-truth network telemetry the guest cannot suppress.
- **Anti-anti-analysis hardening:** SMBIOS/ACPI patching, realistic VM specs, simulated user activity, randomized MAC OUIs.
- **Vulnerable application lab.** A separate but related capability: one-click provisioning of containers/VMs running known-vulnerable applications (e.g. recent cPanel CVE), for rapid CVE reproduction and observation. Shares infrastructure with Tartarus but is conceptually a distinct tool — could grow into its own product.
- **Automated IOC extraction** from raw telemetry: domains, hashes, registry keys, dropped files.
- **Multi-VM scenarios:** detonate sample with a simulated AD environment, internal network targets, etc.

###
