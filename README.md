# Hamistegan

A Linux sandbox for running and observing malware in a live environment. Uses kernel-level eBPF tracing to capture syscalls, file access, and network activity from an isolated container, then streams the events to a web dashboard in real time.

## Architecture

```
┌─────────────┐      eBPF events        ┌─────────────┐      logs/ws       ┌─────────────┐
│   Sandbox   │ ──────────────────────► │   Backend   │ ─────────────────► │  Frontend   │
(C++ container)                         │  (Node.js)  │                    │ (Dashboard) │
└─────────────┘                         └─────────────┘                    └─────────────┘
      │                                        ▲
      │  namespaces, seccomp,                  │
      │  cgroups, capabilities                 │
      ▼                                        │
┌─────────────┐                         ┌──────┴──────┐
│   Malware   │                         │  Observer   │
│  (isolated) │ ◄── traced by ────────► │ (eBPF loader│
└─────────────┘                         │  + reader)  │
                                        └─────────────┘
```

## Project Structure

```
container/          C++ — sandbox, observer, and eBPF programs
├── sandbox/        Process isolation (namespaces, seccomp, capabilities, cgroups)
├── observer/       Userspace eBPF program loader and event reader
├── ebpf/           Kernel-side eBPF tracing programs
└── common/         Shared headers (RAII utilities, types)
backend/            Node.js — receives events from the observer, serves the API
frontend/           Web dashboard — displays live malware activity logs
```

## Building

### Container (C++)

```bash
cd container
cmake -B build
cmake --build build
```

### Backend

```bash
cd backend
npm install
npm run dev
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```
