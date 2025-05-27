#!/usr/bin/env bash

set -euo pipefail

rm -rf /tmp/rugix-*

function extract_container_rootfs() {
    image=$1
    target=$2
    if [ -e "$target" ]; then
        return 0
    fi
    mkdir -p "$(dirname "$target")"
    container_id=$(docker container create "$image")
    mkdir -p /tmp/rugix-bench-container
    docker export "$container_id" | tar -xf - -C /tmp/rugix-bench-container
    docker container rm "$container_id"
    tar --sort=name --pax-option=exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime -cf "$target" -C /tmp/rugix-bench-container .
}

extract_container_rootfs debian:bookworm-20240812 out/bookworm-old.tar
extract_container_rootfs debian:bookworm-20250520 out/bookworm-new.tar

if [ ! -e "out/bookworm-config.tar" ]; then
    docker build -t bookworm-config-change -f Dockerfile.config-change .
    extract_container_rootfs bookworm-config-change out/bookworm-config.tar
fi

function create_ext4_rootfs() {
    input_tar=$1
    output=$2
    if [ -e "$output" ]; then
        return 0
    fi
    fallocate -l 180M "$output";
    out/mke2fs -F -d "$input_tar" -E hash_seed=035cb65d-0a86-404a-bad7-19c88d05e400 -U 12341234-a4ec-4304-a70f-c549ea829da9 -L root "$output"
}

create_ext4_rootfs "out/bookworm-old.tar" "out/bookworm-old.ext4"
create_ext4_rootfs "out/bookworm-new.tar" "out/bookworm-new.ext4"
create_ext4_rootfs "out/bookworm-config.tar" "out/bookworm-config.ext4"

if [ ! -e "old/xdelta-old-new.vcdiff" ]; then
    xdelta delta out/bookworm-old.ext4 out/bookworm-new.ext4 out/xdelta-old-new.vcdiff || true
fi

if [ ! -e "out/xdelta-old-config.vcdiff" ]; then
    xdelta delta out/bookworm-old.ext4 out/bookworm-config.ext4 out/xdelta-old-config.vcdiff || true
fi

mkdir -p /tmp/rugix-deltar-new
tar -xf out/bookworm-new.tar -C /tmp/rugix-deltar-new
mkdir -p /tmp/rugix-deltar-old
tar -xf out/bookworm-old.tar -C /tmp/rugix-deltar-old
mkdir -p /tmp/rugix-deltar-config
tar -xf out/bookworm-config.tar -C /tmp/rugix-deltar-config

cargo run --bin rugix-delta --release -- copy-planned --casync-block-size 2 --compressor-block-size 16 /tmp/rugix-deltar-new /tmp/rugix-deltar-old >out/deltar-copy.txt
cargo run --bin rugix-delta --release -- copy-planned --casync-block-size 4 --compressor-block-size 16 /tmp/rugix-deltar-new /tmp/rugix-deltar-old >out/deltar-copy.txt
cargo run --bin rugix-delta --release -- copy-planned --casync-block-size 8 --compressor-block-size 16 /tmp/rugix-deltar-new /tmp/rugix-deltar-old >>out/deltar-copy.txt
cargo run --bin rugix-delta --release -- copy-planned --casync-block-size 4 --compressor-block-size 32 /tmp/rugix-deltar-new /tmp/rugix-deltar-old >>out/deltar-copy.txt
cargo run --bin rugix-delta --release -- copy-planned --casync-block-size 8 --compressor-block-size 32 /tmp/rugix-deltar-new /tmp/rugix-deltar-old >>out/deltar-copy.txt
# cargo run --bin rugix-delta --release -- copy-planned /tmp/rugix-deltar-config /tmp/rugix-deltar-old >>out/deltar-copy.txt

# echo "Casync Sizes:" >out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 4 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 8 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 16 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 32 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 64 >>out/sizes.txt

# echo "Fixed Sizes:" >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 4 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 8 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 16 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 32 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-config.ext4" 64 >>out/sizes.txt

# echo "Casync Sizes:" >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 4 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 8 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 16 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 32 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- casync-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 64 >>out/sizes.txt

# echo "Fixed Sizes:" >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 4 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 8 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 16 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 32 >>out/sizes.txt
# cargo run --bin rugix-delta --release -- fixed-delta "out/bookworm-old.ext4" "out/bookworm-new.ext4" 64 >>out/sizes.txt