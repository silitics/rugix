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


# Configurations grouped by chunker
grouped_configs = {
    "fixed": [
        ("fixed-4", 1024),
        ("fixed-4", 2048),
    ],
    "casync": [
        ("casync-8", 1024),
        ("casync-8", 2048),
        ("casync-16", 1024),
        ("casync-16", 2048),
    ],
}

# Colors for each group
colors = {
    "fixed": "#ff7f0e",
    "casync": "#1f77b4",
}

DIVIDER = 28619820 / 100
# DIVIDER = 1024

# Collect data
results = []
labels = []
bar_colors = []

# results.append(188743680 / DIVIDER)
# labels.append("image\n(raw)")
# bar_colors.append("green")

results.append(28619820 / DIVIDER)
labels.append("image\n(xz)")
bar_colors.append("green")


results.append(20654936 / DIVIDER)
labels.append("fixed-4-64\n(1024)")
bar_colors.append("magenta")

results.append(21652312 / DIVIDER)
labels.append("fixed-4-64\n(2048)")
bar_colors.append("magenta")

results.append(15869348 / DIVIDER)
labels.append("fixed-4-32\n(1024)")
bar_colors.append("magenta")

results.append(17336740 / DIVIDER)
labels.append("fixed-4-32\n(2048)")
bar_colors.append("magenta")

for group, configs in grouped_configs.items():
    for chunker, overhead in configs:
        size = run_block_based_benchmark(chunker, overhead)
        percent = size / DIVIDER
        results.append(percent)
        labels.append(f"{chunker}\n({overhead})")
        bar_colors.append(colors[group])

results.append((8977828 + 1191 * 1024) / DIVIDER)
labels.append("deltar-8-16\n(1024)")
bar_colors.append("darkblue")
results.append((8977828 + 1191 * 2048) / DIVIDER)
labels.append("deltar-8-16\n(2048)")
bar_colors.append("darkblue")


results.append(2806433 / DIVIDER)
labels.append("xdelta")
bar_colors.append("purple")


# Plotting
x_indices = np.arange(len(labels))

plt.figure(figsize=(12, 6))
plt.bar(x_indices, results, color=bar_colors, width=0.6)

plt.xlabel("Configuration")
plt.ylabel("Download Size (%)")
plt.title("Delta Update Benchmarks")
plt.xticks(x_indices, labels, rotation=45, ha="right")
plt.grid(True, axis="y")

# Add group labels above bars
# for i, label in enumerate(labels):
#     # group = "fixed" if "fixed" in label else "casync"
#     plt.text(i, results[i] + 1, ha="center", fontsize=8, color=colors[group])

plt.tight_layout()
plt.show()
