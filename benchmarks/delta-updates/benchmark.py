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

RUGIX_DELTA = ROOT_DIR / "target" / "release" / "rugix-delta"

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
    # "bullseye",
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

    jobs = list(itertools.product(SNAPSHOTS, SUITES))
    COMPUTE_POOL.map(compute, jobs)


@compute.command()
def xdelta():
    """
    Compute update sizes with Xdelta3-based static delta updates.
    """

    def compute(job: t.Tuple[str, t.Tuple[str, str], Schedule, str]):
        idx, (src, dst), schedule, suite = job
        results_file = RESULTS_DIR / "xdelta" / suite / schedule.name / f"{dst}.json"
        if results_file.exists():
            print(
                f"Skipping pair ({src}, {dst}) for suite {suite} as it already exists."
            )
            return
        src_build_dir = BUILD_DIR / suite / src
        if not src_build_dir.exists():
            print(f"Skipping snapshot {src} for suite {suite} as it does not exist.")
            return
        dst_build_dir = BUILD_DIR / suite / dst
        if not dst_build_dir.exists():
            print(f"Skipping snapshot {dst} for suite {suite} as it does not exist.")
            return
        src_img_path = src_build_dir / "filesystems" / "partition-4.img"
        dst_img_path = dst_build_dir / "filesystems" / "partition-4.img"
        with tempfile.TemporaryDirectory() as temp_dir:
            patch_path = pathlib.Path(temp_dir) / "delta.vcdiff"
            subprocess.check_call(
                ["xdelta3", "-e", "-s", src_img_path, dst_img_path, patch_path]
            )
            patch_size = patch_path.stat().st_size
        results_file.parent.mkdir(parents=True, exist_ok=True)
        results_file.write_text(
            json.dumps({"src": src, "dst": dst, "patch": patch_size})
        )

    jobs = [
        (idx, pair, schedule, suite)
        for suite in SUITES
        for schedule in SCHEDULES
        for idx, pair in enumerate(schedule.pairs)
    ]
    COMPUTE_POOL.map(compute, jobs)


@d.dataclass(frozen=True)
class BlockBasedStrategy:
    chunker: str
    block_overhead: int
    download_block_size: t.Optional[int] = None

    @property
    def name(self) -> str:
        if self.download_block_size is None:
            return f"{self.chunker}-{self.block_overhead}"
        else:
            return f"{self.chunker}-{self.block_overhead}-{self.download_block_size}"


RAUC = BlockBasedStrategy("fixed-4", 512, 32 * 1024)

CASYNC_64 = BlockBasedStrategy("casync-64", 512)
CASYNC_8 = BlockBasedStrategy("casync-8", 512)


BLOCK_DIFFING_STRATEGIES = [RAUC, CASYNC_64, CASYNC_8]


@compute.command()
def block_diffing():
    """
    Compute update sizes with block diffing.
    """

    def compute(job: t.Tuple[BlockBasedStrategy, t.Tuple[str, str], Schedule, str]):
        strategy, (src, dst), schedule, suite = job
        results_file = (
            RESULTS_DIR
            / "block-diffing"
            / strategy.name
            / suite
            / schedule.name
            / f"{dst}.json"
        )
        if results_file.exists():
            print(
                f"Skipping pair ({src}, {dst}) for suite {suite} as it already exists."
            )
            return
        src_build_dir = BUILD_DIR / suite / src
        if not src_build_dir.exists():
            print(f"Skipping snapshot {src} for suite {suite} as it does not exist.")
            return
        dst_build_dir = BUILD_DIR / suite / dst
        if not dst_build_dir.exists():
            print(f"Skipping snapshot {dst} for suite {suite} as it does not exist.")
            return
        src_img_path = src_build_dir / "filesystems" / "partition-4.img"
        dst_img_path = dst_build_dir / "filesystems" / "partition-4.img"
        cmd = [RUGIX_DELTA, "block-based", "benchmark"]
        if strategy.download_block_size is not None:
            cmd.append("--download-block-size")
            cmd.append(str(strategy.download_block_size))
        cmd.append("--block-overhead")
        cmd.append(str(strategy.block_overhead))
        cmd.append(strategy.chunker)
        cmd.append(src_img_path)
        cmd.append(dst_img_path)
        output = subprocess.check_output(cmd)
        results_file.parent.mkdir(parents=True, exist_ok=True)
        results_file.write_bytes(output)

    def try_job(job):
        try:
            compute(job)
        except Exception as error:
            print(error)

    jobs = [
        (strategy, pair, schedule, suite)
        for strategy in BLOCK_DIFFING_STRATEGIES
        for suite in SUITES
        for schedule in SCHEDULES
        for pair in schedule.pairs
    ]
    COMPUTE_POOL.map(try_job, jobs)


@d.dataclass(frozen=True)
class DeltarStrategy:
    chunker: str
    group_overhead: int
    group_size_limit: int

    @property
    def name(self) -> str:
        return f"{self.chunker}-{self.group_overhead}-{self.group_size_limit}"


DELTAR_STRATEGIES = [
    DeltarStrategy("casync-16", 1024, 32 * 1024),
    # We use 4GiB blocks, i.e., files are never split and a group limit of 1, i.e.,
    # each file gets its own download group.
    DeltarStrategy("fixed-4096", 0, 1),
    DeltarStrategy("fixed-4096", 0, 16 * 1024),
]


@compute.command()
def deltar():
    """
    Compute update sizes with the deltar approach.
    """

    def compute(job: t.Tuple[DeltarStrategy, t.Tuple[str, str], Schedule, str]):
        strategy, (src, dst), schedule, suite = job
        results_file = (
            RESULTS_DIR
            / "deltar"
            / strategy.name
            / suite
            / schedule.name
            / f"{dst}.json"
        )
        if results_file.exists():
            print(
                f"Skipping pair ({src}, {dst}) for suite {suite} as it already exists."
            )
            return
        src_build_dir = BUILD_DIR / suite / src
        if not src_build_dir.exists():
            print(f"Skipping snapshot {src} for suite {suite} as it does not exist.")
            return
        dst_build_dir = BUILD_DIR / suite / dst
        if not dst_build_dir.exists():
            print(f"Skipping snapshot {dst} for suite {suite} as it does not exist.")
            return
        src_tar_path = src_build_dir / "filesystems" / "partition-4.tar"
        dst_tar_path = dst_build_dir / "filesystems" / "partition-4.tar"
        cmd = [
            RUGIX_DELTA,
            "deltar",
            "benchmark",
            "--chunker",
            strategy.chunker,
            "--group-overhead",
            str(strategy.group_overhead),
            "--group-size-limit",
            str(strategy.group_size_limit),
            src_tar_path,
            dst_tar_path,
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
        (strategy, pair, schedule, suite)
        for strategy in DELTAR_STRATEGIES
        for suite in SUITES
        for schedule in SCHEDULES
        for pair in schedule.pairs
    ]
    COMPUTE_POOL.map(try_job, jobs)


@main.group()
def analyze():
    """
    Analyze the computed results.
    """


DEVICES_1K = 1_000
DEVICES_10K = 10_000
DEVICES_100K = 100_000
DEVICES_800K = 800_000

DEVICES = [DEVICES_1K, DEVICES_10K, DEVICES_100K, DEVICES_800K]


@analyze.command()
def base_stats():
    results = {suite: {schedule.name: [] for schedule in SCHEDULES} for suite in SUITES}
    for schedule in SCHEDULES:
        for suite in SUITES:
            for _, new in schedule.pairs:
                results_file = RESULTS_DIR / "base-sizes" / suite / f"{new}.json"
                results[suite][schedule.name].append(
                    json.loads(results_file.read_text())
                )
                results_file = (
                    RESULTS_DIR / "xdelta" / suite / schedule.name / f"{new}.json"
                )
                results[suite][schedule.name][-1]["xdelta"] = json.loads(
                    results_file.read_text()
                )["patch"]
                for strategy in BLOCK_DIFFING_STRATEGIES:
                    results_file = (
                        RESULTS_DIR
                        / "block-diffing"
                        / strategy.name
                        / suite
                        / schedule.name
                        / f"{new}.json"
                    )
                    results[suite][schedule.name][-1][
                        f"block-based-{strategy.name}"
                    ] = json.loads(results_file.read_text())["total_compressed"]
                for strategy in DELTAR_STRATEGIES:
                    results_file = (
                        RESULTS_DIR
                        / "deltar"
                        / strategy.name
                        / suite
                        / schedule.name
                        / f"{new}.json"
                    )
                    results[suite][schedule.name][-1][f"deltar-{strategy.name}"] = (
                        json.loads(results_file.read_text())
                    )

    for suite, suite_results in results.items():
        for schedule_name, schedule_results in suite_results.items():
            total_uncompressed = sum(r["uncompressed"] for r in schedule_results)
            total_compressed = sum(r["compressed"] for r in schedule_results)
            total_xdelta = sum(r["xdelta"] for r in schedule_results)
            total_block_diffing = {
                strategy.name: sum(
                    r[f"block-based-{strategy.name}"] for r in schedule_results
                )
                / 1024
                / 1024
                / 1024
                for strategy in BLOCK_DIFFING_STRATEGIES
            }
            total_deltar_plan = {
                strategy.name: sum(
                    r[f"deltar-{strategy.name}"]["plan"] for r in schedule_results
                )
                / 1024
                / 1024
                / 1024
                for strategy in DELTAR_STRATEGIES
            }
            total_deltar_data = {
                strategy.name: sum(
                    r[f"deltar-{strategy.name}"]["data"] for r in schedule_results
                )
                / 1024
                / 1024
                / 1024
                for strategy in DELTAR_STRATEGIES
            }
            print(f"Suite: {suite} | Schedule: {schedule_name}")
            total_uncompressed_gib = total_uncompressed / 1024 / 1024 / 1024
            total_compressed_gib = total_compressed / 1024 / 1024 / 1024
            total_xdelta_gib = total_xdelta / 1024 / 1024 / 1024
            print(f"  Total Uncompressed: {total_uncompressed_gib / 2:.2f} GiB/year")
            print(f"  Total Compressed: {total_compressed_gib / 2:.2f} GiB/year")
            print(
                f"  Compression Reduction: {(1 - total_compressed / total_uncompressed) * 100:.2f} %"
            )
            print(f"  Total Xdelta: {total_xdelta_gib / 2:.2f} GiB/year")
            for strategy in BLOCK_DIFFING_STRATEGIES:
                print(
                    f"  Total {strategy.name}: {total_block_diffing[strategy.name] / 2:.2f} GiB/year"
                )
            for strategy in DELTAR_STRATEGIES:
                plan = total_deltar_plan[strategy.name] / 2
                data = total_deltar_data[strategy.name] / 2
                total = plan + data
                print(
                    f"  Total Deltar ({strategy.name}): {total:.2f} GiB/year ({plan:.2f} + {data:.2f})"
                )
            print("  Cost AWS S3 Egress (0.09 USD/GiB):")
            for devices in DEVICES:
                print(
                    f"    Uncompressed ({devices // 1000}K): {total_uncompressed_gib * AWS_S3_EGRESS_COST * devices / 100 / 2:.2f} USD/year"
                )
                print(
                    f"    Compressed ({devices // 1000}K): {total_compressed_gib * AWS_S3_EGRESS_COST * devices / 100 / 2:.2f} USD/year"
                )
                print(
                    f"    Xdelta ({devices // 1000}K): {total_xdelta_gib * AWS_S3_EGRESS_COST * devices / 100 / 2:.2f} USD/year"
                )
                for strategy in BLOCK_DIFFING_STRATEGIES:
                    print(
                        f"    {strategy.name} ({devices // 1000}K): {total_block_diffing[strategy.name] * AWS_S3_EGRESS_COST * devices / 100 / 2:.2f} USD/year"
                    )
            print("  Cellular Data Cost (10.00 USD/GiB):")
            for devices in DEVICES:
                print(
                    f"    Uncompressed ({devices // 1000}K): {total_uncompressed_gib * CELLULAR_DATA_COST * devices / 100 / 2:.2f} USD/year"
                )
                print(
                    f"    Compressed ({devices // 1000}K): {total_compressed_gib * CELLULAR_DATA_COST * devices / 100 / 2:.2f} USD/year"
                )
                print(
                    f"    Xdelta ({devices // 1000}K): {total_xdelta_gib * CELLULAR_DATA_COST * devices / 100 / 2:.2f} USD/year"
                )
                for strategy in BLOCK_DIFFING_STRATEGIES:
                    print(
                        f"    {strategy.name} ({devices // 1000}K): {total_block_diffing[strategy.name] * CELLULAR_DATA_COST * devices / 100 / 2:.2f} USD/year"
                    )


if __name__ == "__main__":
    main()
