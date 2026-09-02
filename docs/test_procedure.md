# Test Procedure

This document describes the manual procedure used to start and verify the LiDAR redirection prototype.

The test uses two Docker containers on the same Linux PC:

- `lidar-node` — `172.31.0.10`
- `middleware-node` — `172.31.0.20`

The complete communication path is:

```text
LiDAR Emulator
      ↓
LiDAR Adapter
      ↓
SOME/IP Event
      ↓
someipd
      ↓
gatewayd
      ↓
S-CORE IPC / SHM
      ↓
Rust LiDAR Visualizer
```

---

## 1. Prerequisites

From the repository root:

```bash
cd ~/lidar-redirection-verification
```

The required Docker containers must be available.

Check:

```bash
docker ps -a
```

If the containers have not been created yet, build them according to the repository README.

---

## 2. Start the containers

Using Docker Compose:

```bash
docker-compose up -d
```

Check:

```bash
docker ps
```

Expected containers:

```text
lidar-node
middleware-node
```

Check their IP addresses:

```bash
docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' lidar-node
```

```bash
docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' middleware-node
```

Expected:

```text
172.31.0.10
172.31.0.20
```

---

# 3. Clean middleware runtime state

Open a shell in the middleware container:

```bash
docker exec -it middleware-node bash
```

Stop previously running middleware processes:

```bash
pkill -f gatewayd || true
pkill -f someipd || true
pkill -f lidar_visualizer || true
```

Remove stale runtime state:

```bash
rm -rf /tmp/vsomeip-*
rm -rf /tmp/mw_com-*
rm -rf /dev/shm/lola-*
```

Check the multicast route:

```bash
ip route | grep 224
```

If no multicast route exists, add it:

```bash
ip route add 224.0.0.0/4 dev eth0
```

Keep this container available for the following middleware terminals.

---

# 4. Clean LiDAR runtime state

Open another terminal on the host.

Enter the LiDAR container:

```bash
docker exec -it lidar-node bash
```

Stop previously running LiDAR processes:

```bash
pkill -f lidar_adapter || true
pkill -f lidar_sensor_emulator || true
pkill -f python || true
```

Remove stale SOME/IP runtime state:

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

# 5. Start `gatewayd`

Open a new terminal.

Enter the middleware container:

```bash
docker exec -it middleware-node bash
```

Run:

```bash
cd /app/inc_someip_gateway
bazel run //src/gatewayd:gatewayd_example
```

Keep this terminal running.

Expected result:

```text
gatewayd starts successfully
```

---

# 6. Start `someipd`

Open another terminal.

Enter the middleware container:

```bash
docker exec -it middleware-node bash
```

Set the SOME/IP configuration:

```bash
cd /app/inc_someip_gateway

export VSOMEIP_CONFIGURATION=/app/inc_someip_gateway/config/vsomeip/someipd-container.json
```

Run:

```bash
bazel run //src/someipd:someipd_example
```

Keep this terminal running.

Expected behavior:

- vSomeIP starts successfully.
- SOME/IP Service Discovery becomes active.
- The middleware discovers the LiDAR service.
- The middleware subscribes to the LiDAR eventgroup.

---

# 7. Start the Rust LiDAR visualizer

Open another terminal.

Enter the middleware container:

```bash
docker exec -it middleware-node bash
```

Run:

```bash
cd /app/inc_someip_gateway
bazel run //examples/lidar_visualizer_sim:lidar_visualizer
```

Keep this terminal running.

Expected behavior:

```text
LiDAR visualizer subscribed to internal middleware topic
```

The visualizer is the final application-side consumer in this prototype.

---

# 8. Start the LiDAR adapter

Open another terminal.

Enter the LiDAR container:

```bash
docker exec -it lidar-node bash
```

Set the required environment:

```bash
cd /app

export VSOMEIP_CONFIGURATION=/app/config/vsomeip/adapter-container.json
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
```

Run:

```bash
/app/adapter_someip/build/lidar_adapter
```

Keep this terminal running.

Expected behavior:

```text
Received emulator frame
Published SOME/IP event 0x8778
```

---

# 9. Start the LiDAR emulator

Open another terminal.

Enter the LiDAR container:

```bash
docker exec -it lidar-node bash
```

Run:

```bash
cd /app
bash scripts/run_emulator.sh
```

Expected behavior:

```text
Sent frame at time ...
```

The emulator should continuously generate LiDAR frames.

---

# 10. Verify SOME/IP Service Discovery

The LiDAR service uses:

```text
Service ID:     0x1234
Instance ID:    0x5678
Event ID:       0x8778
Eventgroup ID:  0x8778
```

Service Discovery uses:

```text
Multicast: 224.224.224.245
Port:      30490
```

The middleware should discover the service advertised by `lidar-node`.

The previous `unknown eventgroup` problem is considered resolved when the subscription becomes active.

---

# 11. Verify SOME/IP event reception

After the emulator starts sending frames, `someipd` should receive events.

Expected information:

```text
service=0x1234
event=0x8778
```

This confirms that the LiDAR-side SOME/IP provider and middleware-side SOME/IP consumer are communicating.

---

# 12. Verify gateway forwarding

`gatewayd` should receive the SOME/IP event from `someipd` and forward it through the S-CORE internal communication mechanism.

Expected behavior:

```text
Forwarded event 0x8778 to IPC subscribers
```

This confirms the transition:

```text
External SOME/IP
      ↓
S-CORE gateway
      ↓
Internal IPC / SHM
```

---

# 13. Verify the visualizer

The Rust visualizer should receive the forwarded sample.

Expected behavior:

```text
LiDAR visualizer subscribed to internal middleware topic
```

followed by received LiDAR frame data.

At this stage, the complete communication chain has been verified.

---

# 14. End-to-end verification

The test is considered successful when the following sequence can be observed:

```text
1. Emulator generates a LiDAR frame
             ↓
2. Adapter receives the frame
             ↓
3. Adapter publishes SOME/IP event 0x8778
             ↓
4. someipd receives the SOME/IP event
             ↓
5. gatewayd forwards the event to S-CORE IPC
             ↓
6. Rust visualizer receives the sample
```

The final path is:

```text
┌─────────────────┐
│ LiDAR Emulator  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ LiDAR Adapter   │
└────────┬────────┘
         │ SOME/IP
         ▼
┌─────────────────┐
│    someipd      │
└────────┬────────┘
         ▼
┌─────────────────┐
│    gatewayd     │
└────────┬────────┘
         │ S-CORE IPC / SHM
         ▼
┌─────────────────┐
│ Rust Visualizer │
└─────────────────┘
```

---

# 15. What this test proves

A successful test proves:

- The two Docker nodes can communicate.
- SOME/IP Service Discovery works across the Docker bridge.
- The LiDAR adapter can publish the LiDAR event.
- `someipd` receives the external SOME/IP event.
- `gatewayd` redirects the event into the S-CORE internal communication path.
- The Rust visualizer receives the forwarded data.

The test does **not** yet prove complete semantic reconstruction of the LiDAR point cloud.

The payload serialization/deserialization remains part of the next development stage.
