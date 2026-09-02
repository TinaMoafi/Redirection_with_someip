# LiDAR Redirection Verification Prototype

Verification prototype for LiDAR point-cloud redirection through SOME/IP and Eclipse S-CORE.

The purpose of this repository is to verify a realistic SDV external-communication path before moving to the next development stage with real microservices, finalized data models, and a more complete SDV deployment.

The prototype demonstrates that LiDAR data can be generated on an external sensor-side node, converted to SOME/IP, received by the S-CORE SOME/IP gateway, redirected through the internal middleware IPC mechanism, and consumed by an application.

---

## 1. What this prototype verifies

The prototype verifies the following end-to-end communication path:

```text
LiDAR emulator
      |
      v
LiDAR adapter
      |
      | SOME/IP
      v
   someipd
      |
      v
   gatewayd
      |
      | S-CORE internal IPC / SHM
      v
Rust LiDAR visualizer
```

The important result is that the complete redirection path is operational.

The prototype is **not yet a final LiDAR application**. The current payload is forwarded successfully, but its serialization/deserialization is not yet finalized. The visualizer therefore proves data delivery rather than complete semantic reconstruction of the original point cloud.

---

## 2. Architecture

The prototype uses two Docker containers running on one Linux PC.

The containers represent two logically separate SDV nodes.

```text
                         Docker bridge network
                           172.31.0.0/24
                                  |
        ---------------------------------------------------
        |                                                 |
        |                                                 |
        v                                                 v

┌──────────────────────┐                    ┌──────────────────────┐
│      lidar-node      │                    │   middleware-node     │
│      172.31.0.10     │                    │      172.31.0.20     │
│                      │                    │                      │
│  LiDAR emulator      │                    │  Eclipse S-CORE      │
│        │             │                    │                      │
│        v             │                    │  someipd             │
│  LiDAR adapter       │                    │      │               │
│        │             │                    │      v               │
│        v             │                    │  gatewayd             │
│  vSomeIP provider    │                    │      │               │
│                      │                    │      │ S-CORE IPC     │
└──────────┬───────────┘                    │      v               │
           │                                │  Rust visualizer     │
           │ SOME/IP                        │                      │
           └────────────────────────────────>│                      │
                                            └──────────────────────┘
```

### LiDAR node

Container:

```text
lidar-node
172.31.0.10
```

Contains:

- LiDAR sensor emulator
- LiDAR adapter
- vSomeIP provider

Responsibilities:

1. Generate LiDAR point-cloud frames.
2. Send frames to the adapter.
3. Convert each frame into a SOME/IP event.
4. Publish the event over the Docker network.

### Middleware node

Container:

```text
middleware-node
172.31.0.20
```

Contains:

- Eclipse S-CORE `inc_someip_gateway`
- `someipd`
- `gatewayd`
- Rust `lidar_visualizer`

Responsibilities:

1. Discover and receive the external SOME/IP service.
2. Receive LiDAR SOME/IP events through `someipd`.
3. Redirect the received data through `gatewayd`.
4. Publish/forward the data through S-CORE internal IPC.
5. Allow the Rust visualizer to consume the resulting samples.

---

## 3. Why SOME/IP is used

The LiDAR source is treated as an external communication node rather than as a local middleware application.

Therefore, the LiDAR data is not sent directly into the internal S-CORE IPC layer.

Instead:

```text
External sensor
      |
      | network communication
      v
    SOME/IP
      |
      v
 S-CORE SOME/IP gateway
      |
      v
Internal middleware IPC
      |
      v
Application
```

This provides a more realistic SDV communication boundary.

Point-cloud data is streaming sensor data, so the SOME/IP event/eventgroup mechanism is used instead of a request/response interaction.

---

## 4. SOME/IP contract

The current prototype uses the following fixed SOME/IP contract:

| Parameter | Value |
|---|---|
| Service ID | `0x1234` |
| Instance ID | `0x5678` |
| Event ID | `0x8778` |
| Eventgroup ID | `0x8778` |
| SD multicast | `224.224.224.245` |
| SD port | `30490` |
| Event UDP port | `30509` |

The provider and consumer configurations must use matching values.

---

## 5. Repository structure

```text
Redirection_with_someip/
│
├── README.md
├── docker-compose.yml
│
├── middleware/
│   ├── inc_someip_gateway/
│   ├── configs/
│   ├── contracts/
│   └── ...
│
├── pointcloud_redirection/
│   ├── adapter_someip/
│   ├── emulator/
│   ├── contracts/
│   ├── config/
│   ├── docker/
│   │   ├── lidar-node.Dockerfile
│   │   └── tools-node.Dockerfile
│   ├── scripts/
│   └── ...
│
└── docs/
    ├── test_procedure.md
    ├── known_issues.md
    └── roadmap.md
```

---

# 6. Prerequisites

The prototype is intended to run on Linux.

Required:

- Git
- Docker Engine
- Docker Compose

The repository contains the source code, Dockerfiles, configuration and scripts required to build the prototype.

No external source-code directory should be required after cloning the repository.

---

# 7. Quick start

Clone the repository:

```bash
git clone https://github.com/TinaMoafi/Redirection_with_someip.git
cd Redirection_with_someip
```

Build the containers:

```bash
docker compose build
```

Start the containers:

```bash
docker compose up -d
```

Check the containers:

```bash
docker ps
```

Expected logical setup:

```text
lidar-node
middleware-node
```

If the host uses the legacy Compose command, use:

```bash
docker-compose build
docker-compose up -d
```

The detailed startup and verification procedure is available in:

```text
docs/test_procedure.md
```

---

# 8. Test procedure

The prototype should be started in the following order:

```text
1. gatewayd
2. someipd
3. lidar_visualizer
4. lidar_adapter
5. LiDAR emulator
```

This makes it possible to verify each communication stage independently.

---

## 8.1 Clean middleware runtime state

Open a shell in the middleware container:

```bash
docker exec -it middleware-node bash
```

Stop previously running processes:

```bash
pkill -f gatewayd || true
pkill -f someipd || true
pkill -f lidar_visualizer || true
```

Clean stale runtime files:

```bash
rm -rf /tmp/vsomeip-*
rm -rf /tmp/mw_com-*
rm -rf /dev/shm/lola-*
```

Check multicast routing:

```bash
ip route | grep 224
```

If no multicast route exists:

```bash
ip route add 224.0.0.0/4 dev eth0
```

---

## 8.2 Clean LiDAR runtime state

Open a shell in the LiDAR container:

```bash
docker exec -it lidar-node bash
```

Stop previous processes:

```bash
pkill -f lidar_adapter || true
pkill -f lidar_sensor_emulator || true
pkill -f python || true
```

Clean stale SOME/IP state:

```bash
rm -rf /tmp/vsomeip-*
```

Check multicast routing:

```bash
ip route | grep 224
```

If no multicast route exists:

```bash
ip route add 224.0.0.0/4 dev eth0
```

---

# 9. Start the middleware node

### Terminal 1 — gatewayd

```bash
docker exec -it middleware-node bash
```

Then:

```bash
cd /app/inc_someip_gateway
bazel run //src/gatewayd:gatewayd_example
```

### Terminal 2 — someipd

```bash
docker exec -it middleware-node bash
```

Then:

```bash
cd /app/inc_someip_gateway

export VSOMEIP_CONFIGURATION=/app/inc_someip_gateway/config/vsomeip/someipd-container.json

bazel run //src/someipd:someipd_example
```

### Terminal 3 — visualizer

```bash
docker exec -it middleware-node bash
```

Then:

```bash
cd /app/inc_someip_gateway

bazel run //examples/lidar_visualizer_sim:lidar_visualizer
```

---

# 10. Start the LiDAR node

### Terminal 4 — LiDAR adapter

```bash
docker exec -it lidar-node bash
```

Then:

```bash
cd /app

export VSOMEIP_CONFIGURATION=/app/config/vsomeip/adapter-container.json
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

/app/adapter_someip/build/lidar_adapter
```

### Terminal 5 — LiDAR emulator

```bash
docker exec -it lidar-node bash
```

Then:

```bash
cd /app
bash scripts/run_emulator.sh
```

---

# 11. Expected verification

The following sequence should be visible in the logs.

### 1. Emulator

The emulator generates LiDAR frames:

```text
Sent frame at time ...
```

### 2. Adapter

The adapter receives the frame:

```text
Received emulator frame
```

and publishes:

```text
Published SOME/IP event 0x8778
```

### 3. SOME/IP Service Discovery

The middleware discovers the LiDAR service:

```text
Service ID: 0x1234
Instance ID: 0x5678
```

The Service Discovery traffic uses:

```text
224.224.224.245:30490
```

### 4. SOME/IP event reception

`someipd` receives the event:

```text
Received SOME/IP event:
service=0x1234
event=0x8778
```

### 5. Gateway

`gatewayd` forwards the event into the internal communication path:

```text
Forwarded event 0x8778 to IPC subscribers
```

### 6. Visualizer

The Rust visualizer receives the forwarded data:

```text
LiDAR visualizer subscribed to internal middleware topic
```

and prints the received frame data.

---

# 12. End-to-end verification

The complete verified path is:

```text
LiDAR emulator
      |
      | point-cloud frame
      v
LiDAR adapter
      |
      | SOME/IP event 0x8778
      v
someipd
      |
      v
gatewayd
      |
      | S-CORE IPC / SHM
      v
lidar_visualizer
```

A successful milestone requires all stages to be operational.

The test demonstrates:

```text
LiDAR frame generated
        ↓
LiDAR frame received by adapter
        ↓
SOME/IP event published
        ↓
SOME/IP Service Discovery
        ↓
SOME/IP event received by someipd
        ↓
Event forwarded by gatewayd
        ↓
IPC sample received by visualizer
```

---

# 13. Current limitation

The current prototype verifies **communication and redirection**, not complete point-cloud semantic reconstruction.

The payload reaches the visualizer, but the payload format is not yet finalized.

Therefore, some values displayed by the visualizer may currently appear as raw payload bytes or loosely interpreted metadata.

This does not invalidate the current milestone.

The key verification is:

```text
The LiDAR frame reaches the final application through
the complete external SOME/IP → S-CORE IPC path.
```

Proper LiDAR serialization/deserialization is the next development step.

See:

```text
docs/known_issues.md
docs/roadmap.md
```

---

# 14. Verification status

| Stage | Status |
|---|---|
| Two-node architecture | PASS |
| Docker networking | PASS |
| LiDAR container | PASS |
| Middleware container | PASS |
| LiDAR emulator | PASS |
| LiDAR adapter | PASS |
| SOME/IP provider | PASS |
| SOME/IP Service Discovery | PASS |
| SOME/IP subscription | PASS |
| `someipd` reception | PASS |
| `gatewayd` forwarding | PASS |
| S-CORE IPC | PASS |
| Rust visualizer reception | PASS |
| Final LiDAR payload interpretation | IN PROGRESS |

---

# 15. Next development stage

This repository is a verification baseline.

The next stage should move from simulated components toward real SDV microservices and a defined LiDAR data model.

The main development steps are:

```text
Current prototype
       |
       v
Define LiDAR payload schema
       |
       v
Implement serialization
       |
       v
Implement deserialization
       |
       v
Validate point-cloud integrity
       |
       v
Replace simulation with real microservices
       |
       v
Performance testing
       |
       v
Distributed/multi-node deployment
       |
       v
Fault handling and production hardening
```

See:

```text
docs/roadmap.md
```

---

# 16. Important implementation notes

The Docker containers use fixed internal IP addresses:

```text
lidar-node       172.31.0.10
middleware-node  172.31.0.20
```

These addresses belong to the Docker network and are not required to exist on the host PC.

The Docker network uses:

```text
172.31.0.0/24
```

The LiDAR and middleware containers require the appropriate network capabilities for multicast routing.

The prototype also requires cleanup of stale SOME/IP and S-CORE runtime state when restarting components manually.

For the complete procedure, see:

```text
docs/test_procedure.md
```
