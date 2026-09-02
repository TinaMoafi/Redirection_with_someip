#!/bin/bash
set -e

# Clean up any stale vsomeip sockets from an unclean previous shutdown
rm -f /tmp/vsomeip-*

# Ensure multicast route exists (needed for SOME/IP-SD under Docker)
ip route add 224.0.0.0/4 dev eth0 2>/dev/null || true

# Keep container alive for exec-based testing, OR launch adapter directly
exec "$@"