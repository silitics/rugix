#!/bin/bash

set -euo pipefail

rm -rf /etc/machine-id
rm -rf /var/lib/dbus/machine-id
rm -rf /var/log
rm -rf /var/cache/ldconfig/aux-cache
rm -rf /etc/ssh/ssh_host_*
rm -rf /var/lib/apt/extended_states
rm -rf /var/lib/dpkg/status-old
