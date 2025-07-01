#!/usr/bin/env python3

import pathlib
import shutil
import subprocess


BENCHMARK_DIR = pathlib.Path(__file__).parent
BUILD_DIR = BENCHMARK_DIR / "build"

SNAPSHOTS = [
    # 2023
    "20230611T103552Z",
    "20230711T151208Z",
    "20230811T092157Z",
    "20230911T024645Z",
    "20231011T085836Z",
    "20231111T025358Z",
    "20231211T150552Z",
    # 2024
    "20240112T092042Z",  # no snapshot for the 11th of January
    "20240212T035119Z",  # no snapshot for the 11th of February
    "20240311T214105Z",
    "20240411T023603Z",
    "20240511T023812Z",
    "20240611T023014Z",
    "20240711T025212Z",
    "20240811T024124Z",
    "20240911T023716Z",
    "20241011T023255Z",
    "20241111T025602Z",
    "20241211T030016Z",
    # 2025
    "20250111T025329Z",
    "20250211T025012Z",
    "20250311T030104Z",
    "20250411T024939Z",
    "20250511T024247Z",
    "20250611T033537Z",
]

SUITES = [
    "bookworm",
    "bullseye",
]

LAYER_CONFIG_TEMPLATE = """
root = true

recipes = [
    "core/debian-bootstrap",
    "core/debian-grub-setup",
    "core/ssh",
    "core/set-hostname",
    "make-reproducible",
    "packages",
]

[parameters."core/debian-bootstrap"]
suite = "@@SUITE@@"
snapshot = "@@SNAPSHOT@@"

[parameters."core/set-hostname"]
hostname = "rugix"
"""


BUILD_DIR.mkdir(parents=True, exist_ok=True)

LAYER_CONFIG_PATH = BENCHMARK_DIR / "rugix-project" / "layers" / "snapshot.toml"

for snapshot in SNAPSHOTS:
    for suite in SUITES:
        build_dir = BUILD_DIR / suite / snapshot
        if build_dir.exists():
            print(
                f"Skipping snapshot {snapshot} for suite {suite} as it already exists."
            )
            continue
        try:
            print(f"Building snapshot {snapshot} for suite {suite}...")
            LAYER_CONFIG_PATH.write_text(
                LAYER_CONFIG_TEMPLATE.replace("@@SNAPSHOT@@", snapshot).replace(
                    "@@SUITE@@", suite
                )
            )
            subprocess.check_call(
                [
                    "./run-bakery",
                    "bake",
                    "image",
                    "snapshot",
                    "--release-version",
                    "20250701000000",
                    "--source-date",
                    "2025-07-01T00:00:00Z",
                ],
                cwd=BENCHMARK_DIR / "rugix-project",
            )
            build_dir.parent.mkdir(parents=True)
            shutil.move(
                BENCHMARK_DIR / "rugix-project" / "build" / "snapshot", build_dir
            )
        except Exception as error:
            print(f"Error building snapshot {snapshot} for suite {suite}:\n{error}")
