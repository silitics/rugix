#!/usr/bin/env python3

import json
import subprocess
import os

import numpy as np
import matplotlib.pyplot as plt

use_config = False


def run_block_based_benchmark(chunker: str, block_overhead: int):
    # Cache path includes both chunker and overhead
    if use_config:
        cache_path = f"out/{chunker}-overhead-{block_overhead}-config.benchmark.json"
    else:
        cache_path = f"out/{chunker}-overhead-{block_overhead}.benchmark.json"

    if os.path.exists(cache_path):
        print(f"[cache] {chunker} (overhead {block_overhead})")
        with open(cache_path, "r") as f:
            return json.load(f)["total_compressed"]

    output = subprocess.check_output(
        [
            "cargo",
            "run",
            "--release",
            "--bin",
            "rugix-delta",
            "--",
            "block-based",
            "benchmark",
            chunker,
            "--block-overhead",
            str(block_overhead),
            "out/bookworm-old.ext4",
            "out/bookworm-new.ext4",
        ]
    )
    print(output)
    result = json.loads(output)

    # Cache the full result to disk
    with open(cache_path, "w") as f:
        json.dump(result, f)

    return result["total_compressed"]


# Parameters
x = [4, 8, 16, 32, 64]
block_overheads = [0, 512, 1024, 2048]
chunkers = ["fixed", "casync"]

# Prepare results: dict[chunker][overhead] = list of results
results = {chunker: {} for chunker in chunkers}
for chunker in chunkers:
    for overhead in block_overheads:
        results[chunker][overhead] = [
            run_block_based_benchmark(f"{chunker}-{val}", overhead) / 28619820 * 100
            for val in x
        ]

# Plotting
x_indices = np.arange(len(x))
total_bars = len(chunkers) * len(block_overheads)
width = 0.8 / total_bars  # Adjust width based on number of bars per group

plt.figure(figsize=(10, 6))

# Define bar positions and labels
bar_labels = []
for i, chunker in enumerate(chunkers):
    for j, overhead in enumerate(block_overheads):
        offset = (i * len(block_overheads) + j - total_bars / 2 + 0.5) * width
        y_values = results[chunker][overhead]
        plt.bar(
            x_indices + offset,
            y_values,
            width=width,
            label=f"{chunker} ({overhead})",
        )

plt.xlabel("Block Size (KiB)")
plt.ylabel("Download Size (%)")
plt.title("Block-Based Delta Updates")
plt.xticks(x_indices, x)
plt.legend()
plt.grid(True, axis="y")
plt.tight_layout()
plt.show()
