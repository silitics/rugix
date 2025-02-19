#!/bin/bash

set -euo pipefail

systemctl enable systemd-timesyncd.service
