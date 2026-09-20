#!/usr/bin/env bash

# Reports the Rust and repository-linting tool versions on container start.

set -euo pipefail

echo "Development tools"
echo "-----------------"
rustc --version
cargo --version
pre-commit --version
