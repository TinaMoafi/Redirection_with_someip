# Known Issues

## 1. LiDAR payload serialization is not finalized

The complete communication path is working, but the LiDAR payload format is not yet finalized.

The payload reaches the visualizer, but some values are currently interpreted as raw bytes or loosely interpreted metadata.

This is expected for the current verification milestone.

The current objective is to prove:

```text
LiDAR frame
    ↓
SOME/IP
    ↓
S-CORE gateway
    ↓
Internal IPC
    ↓
Application
```

rather than to provide the final point-cloud data model.

### Next step

Define a stable LiDAR frame format and implement matching serialization/deserialization.

---

## 2. Visualizer is currently a verification application

The Rust visualizer is used to verify that data reaches the application side of the middleware.

It is not yet a complete production LiDAR visualization or processing service.

---

## 3. Multicast routing must be available

SOME/IP Service Discovery requires multicast communication.

The containers use:

```bash
ip route add 224.0.0.0/4 dev eth0
```

when the multicast route is not already present.

The container therefore requires the network capability needed to add the route.

---

## 4. Stale runtime files can prevent clean restarts

Manual restarts may fail if stale runtime files remain.

The following cleanup is used during testing:

```bash
rm -rf /tmp/vsomeip-*
rm -rf /tmp/mw_com-*
rm -rf /dev/shm/lola-*
```

These files are runtime state and should not be committed to Git.

---

## 5. SOME/IP eventgroup configuration must match

The provider and consumer must use the same eventgroup.

Current contract:

```text
Event ID:       0x8778
Eventgroup ID:  0x8778
```

A mismatch can result in messages such as:

```text
unknown eventgroup
```

---

## 6. Current deployment uses one physical PC

The two containers represent logically separate nodes, but both currently run on the same physical Linux PC.

This verifies logical node separation and network communication but does not yet validate:

- physical Ethernet communication
- switch behavior
- real ECU-to-ECU communication
- network latency under physical conditions
- packet loss on a real network

---

## 7. Performance has not yet been characterized

The current milestone does not provide final measurements for:

- end-to-end latency
- throughput
- frame rate
- jitter
- CPU usage
- memory usage
- packet loss

These measurements should be performed after the LiDAR payload format is finalized.

---

## 8. Fault recovery has not yet been fully tested

The following cases still require dedicated testing:

- emulator restart
- adapter restart
- `someipd` restart
- `gatewayd` restart
- visualizer restart
- temporary network interruption
- malformed payload
- missing frame
- high-rate point-cloud traffic

---

## 9. Current prototype is not production-ready

This repository is a verification prototype and integration baseline.

It should not yet be considered a production deployment.

Production-oriented work still includes:

- finalized payload schema
- serialization/deserialization
- performance characterization
- fault handling
- configuration management
- service lifecycle management
- monitoring
- security
- distributed deployment
