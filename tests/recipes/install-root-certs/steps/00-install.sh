#!/bin/bash

set -euo pipefail

mkdir -p /etc/rugix
cp "${RUGIX_PROJECT_DIR}/keys/root.crt" /etc/rugix/root.crt
cp "${RUGIX_PROJECT_DIR}/keys/other-root.crt" /etc/rugix/other-root.crt