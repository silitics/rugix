#!/bin/bash

set -euo pipefail

export DEBIAN_FRONTEND=noninteractive

systemctl disable userconfig
systemctl disable resize2fs_once

# Only exists on older (Debuan Buster) releases.
systemctl disable unattended-upgrades.service || true

# Only exists on newer (Debian Bookworm) releases.
systemctl disable sshswitch.service || true

apt-get purge -y userconf-pi
rm -f /etc/ssh/sshd_config.d/rename_user.conf
