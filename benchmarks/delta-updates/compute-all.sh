#!/usr/bin/env bash

set -euo pipefail

python3 benchmark.py compute xdelta
python3 benchmark.py compute deltar
python3 benchmark.py compute block-diffing
