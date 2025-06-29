#!/usr/bin/env bash

set -euo pipefail

rm -rf .rugix
RUGIX_DEV=true ./run-bakery bake image old-debian-snapshot --release-version 20250629165915 --source-date-epoch 1751214002
mv build/old-debian-snapshot/filesystems build/filesystems-old
rm -rf .rugix
rm -rf build/old-debian-snapshot/
RUGIX_DEV=true ./run-bakery bake image old-debian-snapshot --release-version 20250629165915 --source-date-epoch 1751214002
mv build/old-debian-snapshot/filesystems build/filesystems-new
sha256sum build/filesystems-*/*.tar