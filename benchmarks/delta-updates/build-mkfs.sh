#!/usr/bin/env bash

set -euo pipefail

mkdir -p out
cd out
wget https://git.kernel.org/pub/scm/fs/ext2/e2fsprogs.git/snapshot/e2fsprogs-1.47.2.tar.gz 
tar -xf e2fsprogs-1.47.2.tar.gz
cd e2fsprogs-1.47.2
./configure
make
cp misc/mke2fs ../