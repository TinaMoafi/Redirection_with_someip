# Development Roadmap

This project is a verification baseline for LiDAR point-cloud redirection through an SDV middleware communication path.

The current milestone focuses on proving the communication architecture.

The next stages move toward real microservices, a defined LiDAR data model, meaningful point-cloud processing, and eventually distributed deployment.

---

## Phase 0 — Communication verification

**Status: Complete**

The current prototype verifies:

```text
LiDAR emulator
      ↓
LiDAR adapter
      ↓
SOME/IP
      ↓
someipd
      ↓
gatewayd
      ↓
S-CORE IPC / SHM
      ↓
Rust visualizer
```

Verified capabilities:

- Two logically separate Docker nodes
- Docker bridge networking
- SOME/IP Service Discovery
- SOME/IP event subscription
- LiDAR event publication
- `someipd` event reception
- `gatewayd` forwarding
- S-CORE internal IPC
- Application-side sample reception

---

## Phase 1 — Define the LiDAR data model

**Status: Next**

Define a stable and documented LiDAR frame format.

The frame should specify, as applicable:

- frame ID
- timestamp
- point count
- X/Y/Z coordinates
- coordinate representation
- numerical precision
- byte order
- point stride
- intensity
- ring/channel information
- payload version

The format should be versioned so that future changes can be handled explicitly.

### Deliverable

A documented LiDAR frame schema.

---

## Phase 2 — Implement serialization

**Status: Next**

Implement deterministic serialization in the LiDAR adapter.

```text
LiDAR frame
     ↓
Serializer
     ↓
SOME/IP payload
```

The serialized representation should be documented and tested.

---

## Phase 3 — Implement deserialization

**Status: Next**

Implement the corresponding decoder on the middleware/application side.

```text
SOME/IP payload
     ↓
Deserializer
     ↓
LiDAR frame
```

The reconstructed frame should preserve the required information from the original frame.

---

## Phase 4 — Validate payload integrity

**Status: Next**

Add automated tests for:

- normal frames
- empty frames
- single-point frames
- large frames
- invalid payloads
- truncated payloads
- version mismatch
- numerical precision

The basic serialization property should be:

```text
deserialize(serialize(frame)) == frame
```

within the defined numerical tolerance.

---

## Phase 5 — Integrate real microservices

**Status: Planned**

Replace the current simulated/verification components with real SDV microservices where appropriate.

The target architecture remains:

```text
External LiDAR service
        │
        │ SOME/IP
        ▼
S-CORE SOME/IP gateway
        │
        │ Internal IPC
        ▼
LiDAR consumer microservice
```

The current prototype should serve as the communication and integration baseline.

---

## Phase 6 — Real point-cloud processing

**Status: Planned**

Move from payload forwarding to meaningful point-cloud processing.

Target processing chain:

```text
Receive
   ↓
Deserialize
   ↓
Validate
   ↓
Transform / Filter
   ↓
Process
   ↓
Publish
   ↓
Visualize / Consume
```

The exact processing stages will depend on the target SDV use case.

---

## Phase 7 — Performance evaluation

**Status: Planned**

Characterize the system using realistic LiDAR data.

Measure:

- end-to-end latency
- throughput
- frame rate
- jitter
- CPU usage
- memory usage
- packet loss
- payload size

Testing should include different point-cloud sizes and frame rates.

---

## Phase 8 — Distributed deployment

**Status: Planned**

Move from the current single-PC setup:

```text
One Linux PC
│
├── lidar-node
└── middleware-node
```

toward a distributed setup:

```text
LiDAR / Sensor node
        │
        │ Ethernet
        ▼
Middleware / Compute node
        │
        ▼
Consumer microservice
```

The communication should be validated over a real network.

---

## Phase 9 — Fault handling and recovery

**Status: Planned**

Test recovery from:

- emulator failure
- adapter failure
- `someipd` failure
- `gatewayd` failure
- consumer failure
- network interruption
- packet loss
- malformed payload
- missing frames

Define expected recovery behavior for each failure.

---

## Phase 10 — Production-oriented integration

**Status: Future**

Address:

- configuration management
- service lifecycle
- logging
- monitoring
- error handling
- security
- version compatibility
- deployment automation
- resource limits
- recovery mechanisms

At this stage, the prototype should evolve into a reusable SDV communication component rather than remain a verification-only setup.
