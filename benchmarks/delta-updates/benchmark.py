#!/usr/bin/env python3
#
# type: ignore

import dataclasses as d
import typing as t

import concurrent.futures
import itertools
import json
import os
import pathlib
import shutil
import subprocess
import tempfile

import click


BENCHMARK_DIR = pathlib.Path(__file__).parent
BUILD_DIR = BENCHMARK_DIR / "build"
RESULTS_DIR = BENCHMARK_DIR / "results"
ROOT_DIR = BENCHMARK_DIR.parent.parent

RUGIX_BUNDLER = ROOT_DIR / "target" / "release" / "rugix-bundler"

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

AWS_S3_EGRESS_COST = 9  # 0.09 USD/GiB
CELLULAR_DATA_COST = 1000  # 10.00 USD/GiB


BUILD_DIR.mkdir(parents=True, exist_ok=True)

LAYER_CONFIG_PATH = BENCHMARK_DIR / "rugix-project" / "layers" / "snapshot.toml"

MONTHLY = list(zip(SNAPSHOTS, SNAPSHOTS[1:]))
QUARTERLY = list(zip(SNAPSHOTS[::3], SNAPSHOTS[3::3]))
BIANNUALLY = list(zip(SNAPSHOTS[::6], SNAPSHOTS[6::6]))
ANNUALLY = list(zip(SNAPSHOTS[::12], SNAPSHOTS[12::12]))


@d.dataclass(frozen=True)
class Schedule:
    name: str
    pairs: t.List[t.Tuple[str, str]]


SCHEDULES = [
    Schedule("monthly", MONTHLY),
    Schedule("quarterly", QUARTERLY),
    Schedule("biannually", BIANNUALLY),
    Schedule("annually", ANNUALLY),
]


@d.dataclass(frozen=True)
class Version:
    suite: str
    snapshot: str


@d.dataclass(frozen=True)
class Scenario:
    group: str
    schedule: str
    pairs: t.List[t.Tuple[Version, Version]]


SCENARIO_GROUPS = ["bookworm", "bullseye", "bullseye-bookworm"]

# Minor update scenarios.
SCENARIOS = [
    Scenario(
        suite,
        schedule.name,
        [(Version(suite, old), Version(suite, new)) for old, new in schedule.pairs],
    )
    for schedule in SCHEDULES
    for suite in SUITES
]
# Major upgrade scenarios.
SCENARIOS.extend(
    Scenario(
        "bullseye-bookworm",
        schedule.name,
        [
            (Version("bullseye", old), Version("bookworm", new))
            for old, new in schedule.pairs
        ],
    )
    for schedule in SCHEDULES
)


@click.group()
def main():
    pass


@main.command()
def build():
    """
    Build reproducible Debian images for benchmark purposes.
    """
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
                build_dir.parent.mkdir(parents=True, exist_ok=True)
                shutil.move(
                    BENCHMARK_DIR / "rugix-project" / "build" / "snapshot", build_dir
                )
            except Exception as error:
                print(f"Error building snapshot {snapshot} for suite {suite}:\n{error}")


@main.command()
def schedules():
    """
    Print update schedules.
    """
    print("Monthly:")
    for src, dst in MONTHLY:
        print(f"  {src} to {dst}")
    print("Quarterly:")
    for src, dst in QUARTERLY:
        print(f"  {src} to {dst}")
    print("Biannually:")
    for src, dst in BIANNUALLY:
        print(f"  {src} to {dst}")
    print("Annually:")
    for src, dst in ANNUALLY:
        print(f"  {src} to {dst}")


@main.group()
def compute():
    """
    Run benchmarks and store results.
    """


COMPUTE_POOL = concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count())


@compute.command()
def base_sizes():
    """
    Compute base update sizes without delta updates.
    """

    def compute(job: t.Tuple[str, str]):
        snapshot, suite = job
        results_file = RESULTS_DIR / "base-sizes" / suite / f"{snapshot}.json"
        if results_file.exists():
            print(
                f"Skipping snapshot {snapshot} for suite {suite} as it already exists."
            )
            return
        build_dir = BUILD_DIR / suite / snapshot
        if not build_dir.exists():
            print(
                f"Skipping snapshot {snapshot} for suite {suite} as it does not exist."
            )
            return
        img_path = build_dir / "filesystems" / "partition-4.img"
        uncompressed_size = img_path.stat().st_size
        with tempfile.TemporaryDirectory() as temp_dir:
            xz_file_path = pathlib.Path(temp_dir) / "partition-4.img.xz"
            with xz_file_path.open("wb") as xz_file:
                subprocess.check_call(
                    ["xz", "-c", "-z", "-k", img_path], stdout=xz_file
                )
            compressed_size = xz_file_path.stat().st_size
        results_file.parent.mkdir(parents=True, exist_ok=True)
        results_file.write_text(
            json.dumps(
                {"uncompressed": uncompressed_size, "compressed": compressed_size}
            )
        )

    def try_job(job):
        try:
            compute(job)
        except Exception as error:
            print(error)

    jobs = list(itertools.product(SNAPSHOTS, SUITES))
    COMPUTE_POOL.map(try_job, jobs)


@compute.command()
def xdelta():
    """
    Compute update sizes with Xdelta3-based static delta updates.
    """

    def compute(job: t.Tuple[Scenario, t.Tuple[Version, Version]]):
        scenario, (old, new) = job
        results_file = (
            RESULTS_DIR
            / "xdelta"
            / scenario.group
            / scenario.schedule
            / f"{new.snapshot}.json"
        )
        if results_file.exists():
            print(
                f"Skipping pair ({old.snapshot}, {new.snapshot}) for scenario {scenario.group}."
            )
            return
        old_build_dir = BUILD_DIR / old.suite / old.snapshot
        new_build_dir = BUILD_DIR / new.suite / new.snapshot
        old_img_path = old_build_dir / "filesystems" / "partition-4.img"
        new_img_path = new_build_dir / "filesystems" / "partition-4.img"
        with tempfile.TemporaryDirectory() as temp_dir:
            patch_path = pathlib.Path(temp_dir) / "delta.vcdiff"
            subprocess.check_call(
                ["xdelta3", "-e", "-s", old_img_path, new_img_path, patch_path]
            )
            patch_size = patch_path.stat().st_size
        results_file.parent.mkdir(parents=True, exist_ok=True)
        results_file.write_text(json.dumps({"patch": patch_size}))

    def try_job(job):
        try:
            compute(job)
        except Exception as error:
            print(error)

    jobs = [(scenario, pair) for scenario in SCENARIOS for pair in scenario.pairs]
    COMPUTE_POOL.map(try_job, jobs)


@d.dataclass(frozen=True)
class BlockBasedStrategy:
    chunker: str
    group_overhead: int
    group_size: t.Optional[int] = None

    @property
    def name(self) -> str:
        if self.group_size is None:
            return f"{self.chunker}-{self.group_overhead}"
        else:
            return f"{self.chunker}-{self.group_overhead}-{self.group_size}"


RAUC = BlockBasedStrategy("fixed-4", 768, 32 * 1024)
RAUC2 = BlockBasedStrategy("fixed-4", 768, 64 * 1024)

CASYNC_64 = BlockBasedStrategy("casync-64", 768)
CASYNC_16 = BlockBasedStrategy("casync-16", 768)
CASYNC_8 = BlockBasedStrategy("casync-8", 768)


BLOCK_DIFFING_STRATEGIES = [RAUC, RAUC2, CASYNC_64, CASYNC_16, CASYNC_8]


@compute.command()
def block_diffing():
    """
    Compute update sizes with block diffing.
    """

    def compute(job: t.Tuple[BlockBasedStrategy, Scenario, t.Tuple[Version, Version]]):
        strategy, scenario, (old, new) = job
        results_file = (
            RESULTS_DIR
            / "block-diffing"
            / strategy.name
            / scenario.group
            / scenario.schedule
            / f"{new.snapshot}.json"
        )
        if results_file.exists():
            print(
                f"Skipping pair ({old.snapshot}, {new.snapshot}) for scenario {scenario.group}."
            )
            return
        old_build_dir = BUILD_DIR / old.suite / old.snapshot
        new_build_dir = BUILD_DIR / new.suite / new.snapshot
        old_img_path = old_build_dir / "filesystems" / "partition-4.img"
        new_img_path = new_build_dir / "filesystems" / "partition-4.img"
        cmd = [RUGIX_BUNDLER, "simulator", "block-based", "simulate"]
        if strategy.group_size is not None:
            cmd.extend(["--group-size", str(strategy.group_size)])
        cmd.extend(
            [
                "--group-overhead",
                str(strategy.group_overhead),
                "--chunker",
                strategy.chunker,
                old_img_path,
                new_img_path,
            ]
        )
        output = subprocess.check_output(cmd)
        results_file.parent.mkdir(parents=True, exist_ok=True)
        results_file.write_bytes(output)

    def try_job(job):
        try:
            compute(job)
        except Exception as error:
            print(error)

    jobs = [
        (strategy, scenario, pair)
        for strategy in BLOCK_DIFFING_STRATEGIES
        for scenario in SCENARIOS
        for pair in scenario.pairs
    ]
    COMPUTE_POOL.map(try_job, jobs)


@d.dataclass(frozen=True)
class DeltarStrategy:
    chunker: str
    group_overhead: int
    group_size: int

    @property
    def name(self) -> str:
        return f"{self.chunker}-{self.group_overhead}-{self.group_size}"


DELTAR_STRATEGIES = [
    DeltarStrategy("casync-16", 768, 32 * 1024),
    # We use 4GiB blocks, i.e., files are never split and a group limit
    # of 1, i.e., each file gets its own download group.
    DeltarStrategy("fixed-4096", 0, 1),
    DeltarStrategy("fixed-4096", 48, 16 * 1024),
]


@compute.command()
def deltar():
    """
    Compute update sizes with the deltar approach.
    """

    def compute(job: t.Tuple[DeltarStrategy, Scenario, t.Tuple[Version, Version]]):
        strategy, scenario, (old, new) = job
        results_file = (
            RESULTS_DIR
            / "deltar"
            / strategy.name
            / scenario.group
            / scenario.schedule
            / f"{new.snapshot}.json"
        )
        if results_file.exists():
            print(
                f"Skipping pair ({old.snapshot}, {new.snapshot}) for scenario {scenario.group}."
            )
            return
        old_build_dir = BUILD_DIR / old.suite / old.snapshot
        new_build_dir = BUILD_DIR / new.suite / new.snapshot
        old_tar_path = old_build_dir / "filesystems" / "partition-4.tar"
        new_tar_path = new_build_dir / "filesystems" / "partition-4.tar"
        cmd = [
            RUGIX_BUNDLER,
            "simulator",
            "deltar",
            "simulate",
            "--chunker",
            strategy.chunker,
            "--group-overhead",
            str(strategy.group_overhead),
            "--group-size",
            str(strategy.group_size),
            old_tar_path,
            new_tar_path,
        ]
        output = subprocess.check_output(cmd)
        results_file.parent.mkdir(parents=True, exist_ok=True)
        results_file.write_bytes(output)

    def try_job(job):
        try:
            compute(job)
        except Exception as error:
            print(error)

    jobs = [
        (strategy, scenario, pair)
        for strategy in DELTAR_STRATEGIES
        for scenario in SCENARIOS
        for pair in scenario.pairs
    ]
    COMPUTE_POOL.map(try_job, jobs)


@main.command()
def analyze():
    """
    Analyze the computed results.
    """

    base_sizes = {}
    for suite in SUITES:
        base_sizes[suite] = {}
        for snapshot in SNAPSHOTS:
            results_file = RESULTS_DIR / "base-sizes" / suite / f"{snapshot}.json"
            base_sizes[suite][snapshot] = json.loads(results_file.read_text())

    results = {group: {} for group in SCENARIO_GROUPS}
    for scenario in SCENARIOS:
        scenario_results = results[scenario.group][scenario.schedule] = []
        for _, new in scenario.pairs:
            pair_results = {}
            scenario_results.append(pair_results)
            results_file = (
                RESULTS_DIR
                / "xdelta"
                / scenario.group
                / scenario.schedule
                / f"{new.snapshot}.json"
            )
            pair_results["xdelta"] = json.loads(results_file.read_text())["patch"]
            for strategy in BLOCK_DIFFING_STRATEGIES:
                results_file = (
                    RESULTS_DIR
                    / "block-diffing"
                    / strategy.name
                    / scenario.group
                    / scenario.schedule
                    / f"{new.snapshot}.json"
                )
                pair_results[f"block-based-{strategy.name}"] = json.loads(
                    results_file.read_text()
                )["compressed"]
            for strategy in DELTAR_STRATEGIES:
                results_file = (
                    RESULTS_DIR
                    / "deltar"
                    / strategy.name
                    / scenario.group
                    / scenario.schedule
                    / f"{new.snapshot}.json"
                )
                pair_results[f"deltar-{strategy.name}"] = json.loads(
                    results_file.read_text()
                )

    compression_factors = {group: {} for group in SCENARIO_GROUPS}
    total_sizes = {group: {} for group in SCENARIO_GROUPS}

    for scenario in SCENARIOS:
        scenario_results = results[scenario.group][scenario.schedule]
        total_uncompressed = sum(
            base_sizes[new.suite][new.snapshot]["uncompressed"]
            for _, new in scenario.pairs
        )
        total_compressed = sum(
            base_sizes[new.suite][new.snapshot]["compressed"]
            for _, new in scenario.pairs
        )
        total_xdelta = sum(r["xdelta"] for r in scenario_results)
        total_sizes[scenario.group][scenario.schedule] = {}
        total_sizes[scenario.group][scenario.schedule]["uncompressed"] = (
            total_uncompressed
        )
        total_sizes[scenario.group][scenario.schedule]["compression"] = total_compressed
        total_sizes[scenario.group][scenario.schedule]["xdelta"] = total_xdelta
        compression_factors[scenario.group][scenario.schedule] = {}
        compression_factors[scenario.group][scenario.schedule]["compression"] = (
            total_compressed / total_uncompressed
        )
        compression_factors[scenario.group][scenario.schedule]["xdelta"] = (
            total_xdelta / total_uncompressed
        )
        total_block_diffing = {
            strategy.name: sum(
                r[f"block-based-{strategy.name}"] for r in scenario_results
            )
            for strategy in BLOCK_DIFFING_STRATEGIES
        }
        for strategy in BLOCK_DIFFING_STRATEGIES:
            compression_factors[scenario.group][scenario.schedule][
                f"block-based-{strategy.name}"
            ] = total_block_diffing[strategy.name] / total_uncompressed
            total_sizes[scenario.group][scenario.schedule][
                f"block-based-{strategy.name}"
            ] = total_block_diffing[strategy.name]
        total_deltar_plan = {
            strategy.name: sum(
                r[f"deltar-{strategy.name}"]["plan"] for r in scenario_results
            )
            for strategy in DELTAR_STRATEGIES
        }
        total_deltar_data = {
            strategy.name: sum(
                r[f"deltar-{strategy.name}"]["data"] for r in scenario_results
            )
            for strategy in DELTAR_STRATEGIES
        }
        for strategy in DELTAR_STRATEGIES:
            compression_factors[scenario.group][scenario.schedule][
                f"deltar-{strategy.name}"
            ] = (
                total_deltar_plan[strategy.name] + total_deltar_data[strategy.name]
            ) / total_uncompressed
            compression_factors[scenario.group][scenario.schedule][
                f"deltar-{strategy.name}-plan"
            ] = total_deltar_plan[strategy.name] / total_uncompressed
            compression_factors[scenario.group][scenario.schedule][
                f"deltar-{strategy.name}-data"
            ] = total_deltar_data[strategy.name] / total_uncompressed
            total_sizes[scenario.group][scenario.schedule][
                f"deltar-{strategy.name}"
            ] = total_deltar_plan[strategy.name] + total_deltar_data[strategy.name]
            total_sizes[scenario.group][scenario.schedule][
                f"deltar-{strategy.name}-plan"
            ] = total_deltar_plan[strategy.name]
            total_sizes[scenario.group][scenario.schedule][
                f"deltar-{strategy.name}-data"
            ] = total_deltar_data[strategy.name]

    print("Compression Factors:")
    print(json.dumps(compression_factors))
    print("Total Sizes:")
    print(json.dumps(total_sizes))


if __name__ == "__main__":
    main()
